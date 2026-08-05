use futures::channel::mpsc::UnboundedSender;
use gio;
use std::fs::{File, OpenOptions}; // أضفنا OpenOptions لدعم وضع الإلحاق (Append)
use std::ffi::OsStr;
use std::sync::{Arc,Mutex};
//use gio::prelude::DataInputStreamExtManual;
//use gio::prelude::InputStreamExt;
use gio::prelude::*;
use crate::baseplugin::base::OutMesseageType;
use crate::baseplugin::base::DownloadTask;
use crate::baseplugin::base::DownloadTaskTrait;
use crate::baseplugin::base::DownloadFractionInfo;
use crate::baseplugin::base::TempFileDirPath;
use std::io::Write;




pub async fn download_and_save_with_progress_async<F>(
    mut files_to_download: Box<[Arc<Mutex<Option<DownloadTask>>>]>,
    cancellable: gio::Cancellable,
    mut callback: F,
) 
where
    F: FnMut(OutMesseageType),
{
    let client = reqwest::Client::new();
    let mut count: u32 = 0 ;
    let  files_count: u32 = files_to_download.len() as u32 ;
    let mut done_file_path: Vec<TempFileDirPath> = Vec::new(); 
    for task in files_to_download.iter_mut() {
        let mut download_t = task.lock().unwrap();
        count += 1 ;
        if let Some(download_task) = download_t.as_mut() {
            if download_task.generate_download_location() {
                
                let dir = download_task.dir_download_location.clone().unwrap();
                let file_name = std::path::Path::new(&dir).join(&download_task.file_name);
                done_file_path.push(
                    TempFileDirPath {
                        dir_path: download_task.dir_download_location.clone().unwrap(),
                        file_path: file_name.display().to_string(),
                        }
                ); 
                // 1. التحقق من حجم الملف المحلي إذا كان موجوداً
                let mut downloaded: u64 = 0;
                if file_name.exists() {
                    if let Ok(metadata) = std::fs::metadata(&file_name) {
                        downloaded = metadata.len();
                    }
                }

                // 2. بناء الطلب وإضافة ترويسة Range إذا كان هناك جزء محمل مسبقاً
                let mut request = client.get(&*download_task.link);
                if downloaded > 0 {
                    request = request.header("Range", format!("bytes={}-", downloaded));
                }

                // الاتصال بالسيرفر
                let mut response = match request.send().await {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("خطأ في الاتصال: {}", e);
                        callback(OutMesseageType::DownloadError);
                        continue;
                    }
                };

                let total_size: u64;
                let mut dest_file: File;

                // 3. تحليل استجابة السيرفر وتحديد وضع فتح الملف
                if response.status() == reqwest::StatusCode::PARTIAL_CONTENT {
                    // السيرفر يدعم الاستكمال (الرمز 206)
                    let remaining_size = response.content_length().unwrap_or(0);
                    total_size = downloaded + remaining_size;

                    dest_file = match OpenOptions::new().append(true).open(&file_name) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("فشل فتح الملف للإلحاق: {}", e);
                            callback(OutMesseageType::DownloadError);
                            continue;
                        }
                    };
                } else if response.status() == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
                    // الملف مكتمل بالفعل (الرمز 416)
                    if count == files_count {
                        let download_info = DownloadFractionInfo::new(1.0,count,files_count);
                        callback(OutMesseageType::Progress(download_info));
                        callback(OutMesseageType::DownloadState(Some(done_file_path.clone())));
                    }else {
                        let download_info = DownloadFractionInfo::new(0.0,count,files_count);
                        callback(OutMesseageType::Progress(download_info));
                    }
                    continue; // الانتقال للملف التالي
                } else if response.status().is_success() {
                    // السيرفر لا يدعم الاستكمال أو هذا تحميل جديد (الرمز 200)
                    downloaded = 0;
                    total_size = response.content_length().unwrap_or(0);

                    dest_file = match File::create(&file_name) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("فشل إنشاء الملف: {}", e);
                            callback(OutMesseageType::DownloadError);
                            continue;
                        }
                    };
                } else {
                    // خطأ من السيرفر (مثل 404 أو 500)
                    eprintln!("خطأ من السيرفر: {}", response.status());
                    callback(OutMesseageType::DownloadError);
                    continue;
                }

                let mut download_successful = false;

                // 4. حلقة قراءة البيانات
                loop {
                    if cancellable.is_cancelled() {
                        callback(OutMesseageType::DownloadCancelled);
                        // ❌ تم إزالة `remove_file` هنا لكي يبقى الملف ناقصاً ونستكمله لاحقاً
                        return; 
                    }

                    match response.chunk().await {
                        Ok(Some(chunk)) => {
                            if let Err(e) = dest_file.write_all(&chunk) {
                                eprintln!("فشل الكتابة على القرص: {}", e);
                                callback(OutMesseageType::DownloadError);
                                break;
                            }

                            downloaded += chunk.len() as u64;
                            if total_size > 0 {
                                let fraction = (downloaded as f64) / (total_size as f64);
                                let download_info = DownloadFractionInfo::new(fraction,count,files_count);
                                callback(OutMesseageType::Progress(download_info));
                            }
                        }
                        Ok(None) => {
                            download_successful = true;
                            break;
                        }
                        Err(e) => {
                            eprintln!("انقطع الاتصال أثناء التحميل: {}", e);
                            callback(OutMesseageType::DownloadError);
                            break;
                        }
                    }
                }

                // 5. تأكيد الانتهاء
                if download_successful && !cancellable.is_cancelled() && (total_size == 0 || downloaded == total_size) {
                    if count == files_count {
                        let download_info = DownloadFractionInfo::new(1.0,count,files_count);
                        callback(OutMesseageType::Progress(download_info));
                        callback(OutMesseageType::DownloadState(Some(done_file_path.clone())));
                    }else {
                        let download_info = DownloadFractionInfo::new(0.0,count,files_count);
                        callback(OutMesseageType::Progress(download_info));
                    }
                }
            }
        }
    }
}

    
/*pub fn copy_and_save_with_progress_async(
    mut files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>,
    sender: UnboundedSender<OutMesseageType>,
    cancellable: gio::Cancellable,
) {
    for task in &mut files_to_dowmload {
        let clone1_sender = sender.clone();
        let cancellable1_clone = cancellable.clone();
        let mut  download_t = task.lock().unwrap();
        if let Some( download_task) = download_t.as_mut() {
            if download_task.generate_download_location() {
                let src_file  = gio::File::for_uri(download_task.link);
                let file_name = fs::join_paths(&download_task.dir_download_location.clone().unwrap(),download_task.file_name); 
                let dest_file = gio::File::for_path(file_name);

                let clone2_sender = clone1_sender.clone();
                let cancellable2_clone = cancellable1_clone.clone();
                // نحتاج لنسخة إضافية من المرسل لاستخدامها داخل دالة التقدم
                let progress_sender = clone2_sender.clone(); 

                // نبدأ التحميل والنسخ
                src_file.copy_async(
                    &dest_file,
                    gio::FileCopyFlags::OVERWRITE, // استبدال الملف إن وُجد
                    gio::glib::Priority::default(),
                    Some(&cancellable2_clone),
                    // 1. Progress Callback: يتم استدعاؤه بشكل متكرر أثناء التحميل
                    Some(Box::new(move |current_num_bytes: i64, total_num_bytes: i64| {
                        // التأكد من أن الحجم الكلي معروف لتجنب القسمة على صفر
                        if total_num_bytes > 0 {
                            let fraction = current_num_bytes as f64 / total_num_bytes as f64;
                            let _ = progress_sender.unbounded_send(OutMesseageType::Progress(fraction));
                        }
                    })),
                    // 2. Result Callback: يتم استدعاؤه مرة واحدة عند انتهاء العملية أو فشلها
                    move |res| {
                        match res {
                            Ok(_) => {
                                let _ = clone2_sender.unbounded_send(OutMesseageType::Progress(100.0));
                                let _ = clone2_sender.unbounded_send(OutMesseageType::DownloadState(true));
                            }
                            Err(e) => {
                                if e.matches(gio::IOErrorEnum::Cancelled) {
                                    let _ = clone2_sender.unbounded_send(OutMesseageType::DownloadCancelled);
                                } else {
                                    eprintln!("Error downloading file: {:?}", e);
                                    let _ = clone2_sender.unbounded_send(OutMesseageType::DownloadError);
                                }
                            }
                        }
                    }
                );
        }
        }
    }

}*/

pub fn run_command_async_with_output(command: &str, sender: UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
    let vec_command: Vec<&OsStr> = command.split_whitespace().map(|s| OsStr::new(s)).collect();
    let flags = gio::SubprocessFlags::STDOUT_PIPE | gio::SubprocessFlags::STDERR_MERGE;
    
    if let Ok(process) = gio::Subprocess::newv(&vec_command, flags) {
        if let Some(stdout_stream) = process.stdout_pipe() {
        
            fn read_next_chunk(
                stream: gio::InputStream, 
                p_control: gio::Subprocess, 
                sender: UnboundedSender<OutMesseageType>,
                mut internal_buffer: Vec<u8>,
                cancellable: gio::Cancellable,
            ) {
                let clone_sender = sender.clone();
                let process_ref = p_control.clone();
                let stream_clone = stream.clone();
                let cancellable_clone = cancellable.clone();
                stream.read_bytes_async(
                    4096,
                    gio::glib::Priority::default(),
                    //None as Option<&gio::Cancellable>,
                   Some(&cancellable_clone.clone()),
                    move |res| {
                        match res {
                            Ok(bytes) => {

                                if bytes.is_empty() {
                                    if !internal_buffer.is_empty() {
                                        let leftover = String::from_utf8_lossy(&internal_buffer).trim_end().to_string();
                                        if !leftover.is_empty() {
                                            let _ = clone_sender.unbounded_send(OutMesseageType::Message(leftover));
                                        }
                                    }

                                    process_ref.wait_check_async(None as Option<&gio::Cancellable>, move |wait_res| {
                                        if wait_res.is_ok() {
                                            let _ = clone_sender.unbounded_send(OutMesseageType::State(true));
                                        } else {
                                            let _ = clone_sender.unbounded_send(OutMesseageType::State(false));
                                        }
                                    });
                                    return;
                                }
                                
                                internal_buffer.extend_from_slice(&bytes);
                                
                                let mut search_pos = 0;
                                while let Some(newline_idx) = internal_buffer[search_pos..].iter().position(|&b| b == b'\n' || b == b'\r') {
                                    let full_idx = search_pos + newline_idx;
                                    let line_bytes = &internal_buffer[..full_idx];
                                    
                                    let line_str = String::from_utf8_lossy(line_bytes).trim_end().to_string();
                                    if !line_str.is_empty() {
                                        let _ = clone_sender.unbounded_send(OutMesseageType::Message(line_str));
                                    }
                                    
                                    internal_buffer.drain(..=full_idx);
                                    search_pos = 0;
                                }

                                read_next_chunk(stream_clone, process_ref, clone_sender, internal_buffer,cancellable);
                            }
                            Err(e) => {
                                if e.matches(gio::IOErrorEnum::Cancelled) {
                                    let _ = clone_sender.unbounded_send(OutMesseageType::Cancelled);
                                }else {
                                    eprintln!("Error reading chunk: {:?}", e);
                                    let _ = clone_sender.unbounded_send(OutMesseageType::Error);
                                }
                            }
                        }
                    },
                );
            }
            
            read_next_chunk(stdout_stream, process, sender, Vec::new(),cancellable.clone());
        }
    }
}



pub fn run_command(command: &str) -> bool {
    //let vec_command: Vec<&OsStr> = command.split_whitespace().map(OsStr::new).collect();
    let vec_command: &[&OsStr] = &[
            OsStr::new("sh"),
            OsStr::new("-c"),
            OsStr::new(command),
        ];
    
    let flags = gio::SubprocessFlags::STDOUT_SILENCE | gio::SubprocessFlags::STDERR_SILENCE;
    
    if let Ok(process) = gio::Subprocess::newv(&vec_command, flags) {
        if process.wait(gio::Cancellable::NONE).is_ok() {
            return process.exit_status() == 0;
        }
    }
    
    false
}
