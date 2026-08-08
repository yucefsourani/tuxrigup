# tuxrigup
The Essential Post-Setup &amp; Workstation Tuning Suite for Fedora Linux.

## ⚠️ Work in Progress / Development Note

This project is currently under active development. It is an effort to rewrite the [arfedora-welcome](https://github.com/yucefsourani/arfedora-welcome) application from Python to Rust, along with some modifications.

The primary goal of this project was hands-on learning to explore **Rust**, **GTK4**, and **libadwaita**. Because the main focus was understanding how things work under the hood, helper tools and macros (such as `clone!()`) were intentionally used sparingly or avoided. 

As a result, there is certainly room for refactoring and code cleanup. However, as long as the application functions properly, keeping the codebase straightforward is preferred over overcomplicating things—especially given its original purpose as an educational exercise.


## ⚠️ ملاحظة حول التطوير / مشروع قيد الإنجاز

هذا البرنامج قيد التطوير حالياً، وهو محاولة لإعادة كتابة برنامج [arfedora-welcome](https://github.com/yucefsourani/arfedora-welcome) وتحويله من لغة Python إلى Rust مع إجراء بعض التعديلات.

كان الهدف الأساسي من هذا المشروع هو التعلم والتطبيق العملي لاستكشاف لغة **Rust** وكيفية استخدامها مع **GTK4** و **libadwaita**. ولأن التركيز كان منصباً على فهم كيفية عمل الأمور ودقائقها، تم تجنب أو تقليل استخدام الأدوات والماكرو المساعدة مثل `clone!()`.

بناءً على ذلك، هناك العديد من المواضع التي تحتاجه إلى إصلاح وإعادة هيكلة (Refactoring). ومع ذلك، طالما أن البرنامج يعمل بشكل جيد ويؤدي الغرض، فلا داعي لتعقيد الكود أكثر من اللازم، خاصة أنه مشروع أُنشئ بالأساس لغرض التعلم.


## 🛠️ Building and Installation

This project uses the **Meson** build system along with **Ninja** and `cargo`.

### Dependencies
Ensure you have the following installed:
* `rustc` & `cargo`
* `meson` (>= 0.59.0)
* `ninja`
* `gtk4` development files
* `libadwaita` development files

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yucefsourani/tuxrigup
   cd tuxrigup
   ```

2. Setup the build directory:
   ```bash
   meson setup build
   ```

3. Build the project:
   ```bash
   meson compile -C build
   ```

4. Install system-wide (optional):
   ```bash
   sudo meson install -C build
   ```
