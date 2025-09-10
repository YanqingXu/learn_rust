@echo off
echo 🦀 正在创建 Rust 学习项目目录结构...
echo.

REM 创建基础语法目录
if not exist "src\basics" mkdir "src\basics"
echo ✅ 创建 src\basics 目录

REM 创建所有权系统目录
if not exist "src\ownership" mkdir "src\ownership"
echo ✅ 创建 src\ownership 目录

REM 创建结构体和枚举目录
if not exist "src\structs_enums" mkdir "src\structs_enums"
echo ✅ 创建 src\structs_enums 目录

REM 创建错误处理目录
if not exist "src\error_handling" mkdir "src\error_handling"
echo ✅ 创建 src\error_handling 目录

REM 创建泛型和特征目录
if not exist "src\generics_traits" mkdir "src\generics_traits"
echo ✅ 创建 src\generics_traits 目录

REM 创建集合类型目录
if not exist "src\collections" mkdir "src\collections"
echo ✅ 创建 src\collections 目录

REM 创建函数式编程目录
if not exist "src\functional" mkdir "src\functional"
echo ✅ 创建 src\functional 目录

REM 创建并发编程目录
if not exist "src\concurrency" mkdir "src\concurrency"
echo ✅ 创建 src\concurrency 目录

REM 创建项目练习目录
if not exist "src\projects" mkdir "src\projects"
echo ✅ 创建 src\projects 目录

REM 创建测试目录
if not exist "tests" mkdir "tests"
echo ✅ 创建 tests 目录

REM 创建示例目录
if not exist "examples" mkdir "examples"
echo ✅ 创建 examples 目录

REM 创建基准测试目录
if not exist "benches" mkdir "benches"
echo ✅ 创建 benches 目录

echo.
echo 🎉 目录结构创建完成！
echo.
echo 📚 接下来你可以：
echo    1. 运行 'cargo run' 查看主程序
echo    2. 运行 'cargo run --bin variables' 开始学习变量
echo    3. 运行 'cargo run --bin calculator' 尝试计算器项目
echo    4. 查看 'project_structure.md' 了解详细使用说明
echo.
echo 🚀 开始你的 Rust 学习之旅吧！
pause