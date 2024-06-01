# ArceOS-ATS-INTC项目的网络性能测试程序

该程序用于测试[ArceOS-ATS-INTC](https://github.com/rosy233333/arceos-ats-intc)系统的网络性能测试。

在`ArceOS-ATS-INTC`项目文件夹中，使用`make A=apps/presentation ARCH=riscv64 LOG=error NET=y SMP=4`运行系统的测试程序。该系统中的程序首先会自己测试并打印出任务调度测试的结果，之后运行一个TCP服务器。

此时，使用`cargo run`运行本项目，其会与系统建立多个TCP连接，发送数据，并接收系统计算后返回的结果。本项目的程序会统计单位时间内系统处理请求的数目，并打印测试结果。

在`[ArceOS-ATS-INTC项目文件夹]/apps/presentation`和本项目中，都具有cargo feature `output`。启用该feature，则测试规模更小，且会打印运行过程的测试输出，便于展示系统的正确性；禁用该feature，则测试规模更大且没有测试输出，便于展示系统的性能。