# Lab1 实验Report
## 实现的功能
+ 直接使用 **read_volatile** 和 **write_volatile** 完成了 **trace_request** $= 0$或$1$的要求。
+ 对于 **trace_request** $= 2$的情况，在 **TaskControlBlock** 添加了 **syscall_times** 的一维数组，当作桶来统计不同 **syscall_id** 的次数
## 问题简答
### 程序报错行为
    [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
    [kernel] IllegalInstruction in application, kernel killed it.
    [kernel] IllegalInstruction in application, kernel killed it.
使用的RUSTSBI版本为：**v0.4.1**
### trap.S相关问题
1.  代表内核栈的栈顶。从S态回到U态，或者用户程序刚开始执行时。
2.  + sstatus:决定返回什么特权级 
    + sepc:决定返回到哪条指令
    + sscratch：交换内核态和用户态的栈指针
3. x2寄存器为栈指针sp，跳过 x2 是因为 restore 过程中所有加载指令都以 sp 为基址，sp 必须最后恢复。应用不使用 x4 寄存器，因此无需保存，在恢复上下文的时候也就无需恢复了。
4. 执行后sp为用户态栈指针，sscratch为内核态栈指针
5. sret，因为执行这一条指令后，系统读取 sstatus 之后，才会真正切换状态，之前都只是做数据搬运等工作。
6. 执行后sp为内核态栈指针，sscratch为用户态栈指针
7. 是在在 **__alltraps** 之前，在程序调用 **ecall** 时，就已经完成了状态切换。
## 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无
3.  我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。