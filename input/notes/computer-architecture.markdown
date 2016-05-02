---
title = "Computer Architecture"
published = "April 22, 2016"
excerpt = "Computer architecture"
comments = false
---

<toc/>

Moore's Law states that every 18-24 months we get twice the number of transistors onto the same chip area. Essentially, processor speed doubles, energy/operation halves, and memory capacity doubles. The _memory wall_ thus refers to the fact that latency only improves by 1.1x every 2 years, whereas CPU performance and memory capacity double every two years.

Dynamic power is consumed when there is activity on the circuit, whereas static power is consumed when the system is powered on but idle. The active power is:

$$ P = \frac 1 2 C \cdot V^2 \cdot f \cdot \alpha $$

where $C$ is the capacitance (proportional to chip area), $V$ is the power supply voltage, $f$ is the frequency, and $\alpha$ is the activity factor (e.g. what percent of the processor transistors are actually active).

The fabrication yield can be computed as:

$$ \text {yield} = \frac {\text {working chips}} {\text {chips on wafer}} $$

The two different ways in which the benefits of Moore's Law can be reaped are lower cost: a smaller chip that does the same thing as the larger, previous generation chip, or increased speed for the same cost: new chip with the same area that is faster and more capable for the same cost as the previous generation chip.

The Iron Law for measuring CPU time is computed as:

$$
\begin{align*}
\text {CPU Time} &= \frac {\text {instructions}} {\text {program}} \cdot \frac {\text {cycles}} {\text {instructions}} \cdot \frac {\text {seconds}} {\text {cycles}}\\
&= \frac {\text {seconds}} {\text {program}}
\end{align*}
$$

Amdahl's Law measures the speedup when only a fraction of the program was enhanced. The enhanced fraction refers to a percentage of the original execution time that is affected by the enhancement. The implication of Amdahl's Law is to focus on optimizing the common case.

$$ \text {speedup} = \frac {1} {(1 - \text {enhanced fraction}) + \frac {\text {enhanced fraction}} {\text {enhanced speedup}}} $$

Lhadma's Law cautions that in pursuit of optimizing the common case, the uncommon case shouldn't be slowed down too much.

Diminishing returns with respect to Amdahl's Law refers to the fact that continuing to optimize a specific part of the execution eventually provides less and less gains in speedup, because the optimized portion grows smaller and smaller, accounting for less and less of execution time. Intuitively, it can be considered that the "low hanging fruit" of optimizations will be gone after a while, making it harder to optimize. The practical implication of this is that after optimizing a portion of the execution, one must re-evaluate which portion is now dominant.

# Pipelining

Pipeline stalls can happen due to data dependencies. For example, an instruction reads a register but another instruction farther down the pipeline is set to write to that register. The read instruction would obtain an outdated value if it weren't stalled until after the write completed.

Pipeline flushes may be necessary due to jump instructions. By the time the pipeline recognizes that it's a jump instruction, the instructions that follow it are already inside the pipeline behind the jump instruction's stage. Since those instructions which follow the jump instruction aren't actually going to be executed, they need to be flushed out of the pipeline.

The longer a pipeline is, the higher the cost of branch misprediction, because it means that more instructions will have been loaded into the pipeline which need to be flushed.

## Dependencies

A _control dependency_ is when an instruction depends on a branch, such that the pipeline cannot know whether to load those instructions until it determines the result of the branch. These can be eliminated with branch prediction.

A _data dependency_ is when an instruction depends on data computed by a previous instruction.

A _read-after-write data dependency_ (RAW) is when an instruction has a data dependency because it tries to _read_ a value after it was _written_ by a previous instruction. Instructions with RAW dependencies must stall. Forwarding isn't possible for RAW dependencies because it would imply time travel, i.e. sending the written value to the read that occurred in the past.

A _write-after-write data dependency_ (WAW) is when the order of the writes must be preserved. For example, if adding two registers and storing the result in register A, then subtracting two registers and storing the result in register A, the final value of register A should be the subtraction result.

A _write-after-read data dependency_ (WAR) is when a write must occur until after a register was read by a previous instruction.

Read-after-write dependencies are also known as _true dependencies_ because the read truly depends on the write having occurred. True dependencies can be mitigated by out-of-order execution.

Write-after-read and write-after-write dependencies are also known as _false dependencies_, or _name dependencies_, because the only reason that the dependency exists is because the same register is being used for two different results. For example, in write-after-write, two different instructions may write to the same register.

_Register renaming_ is a way to eliminate false/name dependencies. It establishes two kinds of registers. _Architectural registers_ are ones that the programmer/compiler uses, and _physical registers_ are the actual locations the values go to. Register renaming rewrites the program to use physical registers, and uses a _Register Allocation Table_ (RAT) to map architectural registers to physical registers.

The RAT begins with some predefined architecture-to-physical register mappings. Each instruction's architecture register operands are rewritten to physical registers based on those mappings. Every time an instruction writes to an architectural register, that target architectural register is rewritten into a new/different physical register, and the entry for that architectural register is updated in the RAT.

A _structural dependency_ comes about when there isn't enough hardware to perform an operation in the same cycle. For example, if the current instruction requires an adder but all adders are currently being used, it has to wait.

A _pipeline hazard_ is when a dependency results in incorrect execution. It's important to note that a dependency in a pipeline doesn't automatically result in a pipeline hazard.

In order to resolve hazards caused by control dependencies, it's necessary to flush the dependent instructions.

In order to resolve hazards caused by data dependencies, it's necessary to either stall the dependent instructions or forward the correct values to them.

The reason that instructions are loaded into a pipeline before even knowing if a branch will take place is that in the event that the branch is not taken, no stalls are necessary.

# Branch Prediction

The only basis on which a branch predictor may predict is the current instruction's address, i.e. the program counter (PC). In particular, the branch predictor must determine if the current PC is a taken branch, and if so, what the target PC is.

Given the choice between always predicting that a branch isn't taken, and refusing to predict at all by waiting until the branch predicate is computed, it's much better to always predict that the branch isn't taken. This way, in the worst case, it will not be any more costly than waiting until the predicate is computed, and in the best case it will be significantly faster.

The _predict not-taken_ branch predictor works by always predicting that the branch is taken, which is accomplished by simply implementing the PC as usual.

Since a branch predictor can only base its decision on the instruction PC, without knowing if the current instruction even is a branch (since the instruction hasn't been decoded), whether it's taken, or its offset, the way that a branch predictor can improve its predictions is by remembering the previous results for that same PC.

A _branch table buffer_ (BTB) is a table that maps the current PC to the predicted target PC. If it was a misprediction, the target PC is updated for that current PC. In order to keep the BTB small and the lookup fast, the table is indexed by some of the least significant bits of the PC, since those are more likely to differ from one instruction to the next.

A _branch history table_ (BHT) is a table that maps the current PC to whether the branch was taken (1) or not (0).

The BTB works by indexing into the BHT to determine if the branch is taken. If the branch is taken, index into the BTB to determine the target PC. Otherwise if the branch isn't taken, simply increment the PC as usual. On a misprediction, the BTB and BHT are both updated. If it mispredicted that the branch was taken and it wasn't, only the BHT is updated to reflect that the branch isn't taken.

When indexing the BTB and BHT, the least significant bits are used _except_ for the first few bits that represent the instruction alignment boundaries, because those bits will always be zero. For example, on a 32-bit architecture the instructions are word aligned, which means that all instructions are on 4-byte boundaries, which also means that all instruction addresses are divisible by 4, which means that all instruction addresses end in 00.

The _2-Bit Predictor_ (2BP) (aka _2-Bit Counter_ (2BC)) works by maintaining a counter that counts from 0 (0b00, not taken) to 3 (0b11, taken). If the counter is 1 (0b01) or 2 (0b10), the prediction remains the same as the previous, unless it's a misprediction. The first bit can be interpreted as a _prediction bit_, specifying whether the branch is taken or not. The second bit can be interpreted as a _hysteresis_ (conviction) bit, specifying how "sure" it is of the prediction bit's value.

| Counter | Meaning |
| :------ | :------ |
| 0b00    | strong not-taken |
| 0b01    | weak not-taken |
| 0b10    | weak taken |
| 0b11    | strong taken |

The advantage of the 2-Bit Predictor is that a single anomaly will not completely change the prediction. For example, given a small loop, finishing the loop will cause a misprediction, but it won't change the prediction value to not-taken because the majority of the time it _is_ taken. Instead, it would only change if the misprediction happens again.

The preferred state of a 2-Bit Predictor is one of the weak states, 2 or 3, since there's a one-time misprediction cost in the worst case. If it started on a strong state, such as 0 or 4, then there would need to be two mispredictions to update the prediction.

<img src="http://i.imgur.com/ibzq3pF.png" class="center" />

The pathological case of initializing a 2-Bit Predictor with a weak state is that if it alternates between taking a branch and not taking the branch, then each misprediction will only flip between the weak states, causing a misprediction _every time_. If it had started on a strong state, it would only move to a weak state on a misprediction, which would mean that the prediction would be correct half of the time.

More generally, every branch predictor has a sequence where it will mispredict 100% of the time.

A _history-based predictor_ works by keeping track of the last $N$ branch outcomes to determine the next one.

A 1-Bit History BHT works by storing the branch outcome bit along with two 2-Bit Counters. On any given prediction, the current branch outcome bit is used to determine which 2-Bit Counter to use for the prediction. On a misprediction, the outcome bit is updated to reflect the outcome. Regardless of the outcome, the chosen 2-Bit Counter is updated based on that outcome.

<img src="http://i.imgur.com/gP7Fnjh.png" class="center" />

An entry in the BHT of a 2-Bit History Predictor contains 2 bits of history and 4 x 2-Bit Counters (one for each history configuration).

An N-Bit History Predictor must store for each entry in the BHT, $N$ history bits and $2^N$ 2-Bit Counters, one for each configuration. The 2-Bit Counter that is used for any particular prediction is determined by the history bits.

An N-Bit History Predictor can accurately predict all branch prediction patterns of length $\le N + 1$. An N-Bit History Predictor requires, per entry;

$$N + \text{2-bit counter} \cdot 2^N \text{ history permutations}$$

The _PShare Predictor_ works by storing a private history for each branch and sharing 2-Bit Counters. The history bits are stored in a _Pattern History Table_ (PHT). Each entry is XORed with the PC bits to index into the BHT to obtain the shared 2-Bit Counters. When the 2-Bit Counter is updated, the new history is mapped to this updated counter.

<img src="http://i.imgur.com/TNz1iDw.png" class="center" />

The _GShare Predictor_ works similarly to the PShare Predictor, except that there is a global history. This is useful for _correlated branches_. For example, the following branches are correlated in the sense that if one branch is taken, the other is not:

``` cpp
if (shape == square)
  // ...

if (shape != square)
  // ...
```

The _Tournament Predictor_ works by leveraging multiple predictors, such as GShare and PShare, and using a _Meta-Predictor Table_, which stores entries specifying which of the other predictors is more likely to be correct.

The _Hierarchical Predictor_ works similarly to the Tournament Predictor, except that where the Tournament Predictor uses two good predictors and pays a considerable cost for each, a Hierarchical Predictor uses one "good" predictor and one "OK" predictor.

Whereas in a Tournament Predictor both branches are updated on each branch outcome, in a Hierarchical Predictor, the "OK" predictor is updated on each branch outcome, but the "good" predictor is only updated if the "OK" predictor was not correct.

A real-world Hierarchical Predictor in the Pentium M processors works by maintaining a hierarchy of predictors: 2-Bit Counters, Local, and Global predictors. If the 2-Bit Counter mispredicts, that entry is added to the Local predictor, and likewise Local to Global. The Local and Global predictors maintain a tag array indicating whether or not that PC is covered by that predictor.

A _Return Address Stack_ (RAS) predictor is for predicting the target of a function return. A BTB alone would always remember the previous call's return, which may not be the same call and so would be incorrect. An RAS predictor works by maintaining a stack. On each function call, the return address is pushed. On each function return, the stack is popped. An RAS predictor is necessary aside/separate from the regular program call stack because it's necessary for fast predictions. When the RAS predictor stack is full, pushes wrap around. Like branch prediction, a RAS predictor needs to be usable even before the instruction is determined to be a return instruction (`RET`), which is accomplished by using a simple predictor or pre-decoding the instruction.

_Branch Predication_ refers to executing the instructions on both directions/sides of a branch, so that only half of the work is wasted and discarded. This is primarily useful when it ends up being much faster than the branch misprediction overhead.

_if-conversion_ works by removing an `if` condition where both branches are similar, so that instead the work of both branches is done and the correct result is chosen based on the original condition.

``` cpp
// BEFORE
if ( cond) {
  x = arr[i];
  y = y + 1;
} else {
  x = arr[j];
  y = y - 1;
}
```

``` cpp
// AFTER
x1 = arr[i];
x2 = arr[i];

y1 = y + 1;
y2 = y - 1;

x = cond ? x1 : x2;
y = cond ? y1 : y2;
```

During an if-conversion, the result of the correct branch must be chosen, but doing it by using a branch would defeat the purpose of the if-conversion:

``` cpp
x = cond ? x1 : x2;
```

Instead a conditional move instruction can be used. A _conditional move instruction_ performs a move _only if_ the condition is true. On x86, this refers to the `cmovz`, `cmovnz`, `cmovgt`, etc. family of functions.

``` nasm
r3 = cond
r1 = x1
r2 = x2
movn x, r1, r3 ;; use r1 if cond is != 0 (true)
movz x, r2, r3 ;; use r2 if cond is == 0 (false)
```

_Full branch predication_ refers to adding condition bits to _every_ instruction which specifies whether the instruction actually carries out its operation. Some instructions are used to establish the predicates and the subsequent instructions can be predicated with them so that they only carry out their operation if that predicate holds.

``` nasm
; if r1 == 0 { p1 = 1, p2 = 0 } else { p1 = 0, p2 = 1 }
mp.eqz p1, p2, r1
(p2) add1 r2, r2, 1 ;; only add if p2 == 1
(p1) add1 r3, r3, 1 ;; only add if p1 == 1
```

_Instruction Level Parallelism_ (ILP) refers to what the _Instructions Per Cycle_ (IPC) would be on an ideal processor which can execute an entire instruction in 1 cycle, and can execute any number of instructions in the same cycle _while obeying_ true dependencies. ILP is a property of the program, _not_ the processor, since ILP concerns an _ideal_ processor which doesn't exist.

The ILP of a program is determined by renaming registers and "executing" through the code. For example, given:

``` nasm
add p10, p2, p3
xor p6, p7, p8
mul p5, p8, p9
add p4, p8, p9
sub p11, p10, p5 ;; true dep on: add's p10 and mul's p5
```

The ILP of the above program is:

$$ \frac {\text {5 instructions}} {\text {2 cycles}} = 2.5 \text { ILP} $$

2 cycles are necessary because the first 4 instructions can be done in the same cycle in the ideal processor defined by ILP, but the final instruction would occur in the next cycle due to the true dependencies present in the code.

When determining ILP, structural dependencies are not considered since they concern hardware limitations, but ILP presupposes an ideal processor, that is, every instruction that can possibly execute in the same cycle _will_ execute in the same cycle without having to wait on some resource (e.g. all adders are being used).

When determining ILP, control dependencies are not considered since ILP assumes a perfect, same-cycle branch prediction, so that branches still execute but have no impact on delaying further instructions.

ILP is constrained by the issue-width and the order of execution. If narrow-issue and in-order, ILP is limited by narrow-issue. If wide-issue and in-order, ILP is limited by in-order because it prevents saturation of the issue-width. If wide-issue and out-of-order, ILP is maximized because out-of-order can continue to find instructions to saturate the wide-issue, regardless of their order.

# Out-of-Order Execution

Tomasulo's Algorithm is an instruction scheduling algorithm for out-of-order execution. It determines which instructions have inputs ready so that they could begin in the next cycle, whereas the rest still have to wait for their inputs to be produced. This also includes register renaming.

As instructions are fetched from the instruction queue they are placed in a _reservation station_, where they wait for their parameters to become ready. Existing registers are placed into the instructions that depend on them in the reservation station.

When an instruction's result is computed, it is broadcast on a bus and ends up in the register file for the appropriate destination register, and/or in a reservation station's instruction's operands if needed.

Data resulting from a load (from memory) has an output to broadcast on the bus (sent to the register file or dependent reservation stations). Store (in memory) operations have an input from the load data and computed value bus, so that storing data from registers into memory can be accomplished as soon as the register values become available.

When an instruction is _issued_ it means that it's taken off of the instruction queue and sent to the store unit and reservation station.

When an instruction in a reservation station becomes ready, it is _dispatched_, which means it is sent to the compute unit.

When an instruction has computed its result and is ready to be broadcast, it performs a _write result_ or _broadcast_. In particular, the reservation station producing the result as well as the result itself is broadcast.

On each cycle, different instructions will be in any one of the following phases:

* **issue**: fetch instruction from instruction queue, determine dependencies from RAT
* **capture**: update dependencies with latest results
* **dispatch**: send instruction for execution
* **write result (broadcast)**: send result to reservation stations, write to register file, update RAT

During the _issue phase_, the next instruction in program order (so that register renaming works correctly) is taken from the instruction queue. Then the input registers are identified and their source determined---whether already in the register file or produced by an instruction that hasn't yet finished executing---by looking in the RAT. Then a free reservation station of the correct kind (adder, multiplier, etc.) is found. If all reservation stations are taken, nothing is issued this cycle. The instruction is placed into the free reservation station. The instruction's destination register is tagged so that the result is placed there and future instructions that require the register know which instruction produces it.

The _Register Alias Table_ (RAT) is used so that fetched instructions can identify the reservation stations that they depend on, and so that they can specify that all future instructions that require its destination register should wait for its reservation station (i.e. wait for it to complete). If an entry is empty, it signals to other instructions that they should simply read the value from the actual physical register in the register file because the result has already been computed.

During the _dispatch phase_, the reservation station of the just-completed instruction is freed. All reservation stations that depend on the just-completed instruction's result have the result inserted in the corresponding operand(s). Those instructions that become ready are dispatched to the execution unit. If more than one instruction is ready, we choose the instruction to dispatch based on some policy, such as oldest first, random, or most-dependencies first (difficult to implement/costly).

During the _write result_ (broadcast) phase, the reservation station's tag and result are broadcast on the bus. The result is written to the register file by indexing the RAT with the tag to determine the actual register. The RAT is updated so that the entry that contained the tag is cleared, which signals that the register file should be read instead. Finally, the reservation station with that tag is freed.

When more than one instruction finishes in the same cycle and they need to be broadcast, some heuristic is used to quickly determine which one goes first. Usually a priority is given that is inversely proportional to the speed of the unit. For example, if the multiplier is slower than the divide, the multiplier is given a higher priority. This is because they will have been executing for a longer time, so it's more likely that they have more dependencies on them by now.

A stale result can occur because another instruction clobbered/overwrote the same entry in the RAT. If a stale result is broadcast, the reservation station is updated with the result as usual. However, the RAT is not updated because no further instructions will ever use that result. This is because for the result to be stale, it means that another instruction overwrote its entry in the RAT, and that only occurred because there is a newer instruction that is producing the value for that same register.

In Tomasulo's Algorithm, loads and stores occur in-order.

## Reorder Buffer

Exceptions are a problem in Tomasulo's Algorithm because after returning from the exception handler to the excepting instruction, the instructions that follow might have already executed out-of-order. This may clobber/corrupt the operands of the excepting instruction, which would result in an incorrect result on resumption.

Branch mispredictions are a problem in Tomasulo's Algorithm because out-of-order execution can execute instructions following a branch instruction even if there ended up being a misprediction, in which case those instructions shouldn't have executed.

A _phantom exception_ is an exception that triggered despite a branch misprediction, in which case the excepting instruction should never have executed to begin with.

To ensure proper out-of-order execution, values should be deposited to registers in-order. This way, if it turns out that an instruction shouldn't have executed (i.e. due to exceptions or branch mispredictions), the instruction's destination register will not have been overwritten.

A _reorder buffer_ (ROB) is a buffer that remembers the program order and keeps the result of instructions until they are safe to write. A typical entry contains the value produced by the instruction, a done bit specifying whether the value has been set, and the register to write to.

Two pointers into a reorder buffer (ROB) are maintained. The _commit pointer_ points to the next instruction to be completed, and the _issue pointer_ points to where new instructions should be written. The commit pointer trails after the issue pointer, and both wrap around when needed.

A ROB entry is allocated for a new instruction when the new instruction is issued, same as when a reservation station is acquired for the instruction.

When using an ROB, the RAT should point to the ROB entry instead of the register.

During the _commit phase_ of Tomasulo's Algorithm with an ROB, for all instructions from the commit pointer onward that have results ready, the result is written into the target register and the RAT is updated to point to that target register.

On a branch misprediction in Tomasulo's Algorithm with an ROB, the branch's entry in the ROB is marked as a misprediction and the issue pointer is moved to the same location as the commit pointer, as if aborting those instructions. Each RAT entry is made to point to the corresponding register instead of the ROB, the reservation stations are freed, and the ALUs are emptied without broadcasting the results.

On an exception in Tomasulo's Algorithm with an ROB, the exception is treated as any other result and the actual handling of the exception is delayed until the excepting instruction commits.

When committing an instruction's result, the result is written to the corresponding register, regardless of what the RAT says. However, if the corresponding RAT entry points to the ROB entry, then the RAT entry must be cleared so that further instructions consult the register file instead of the ROB.

A _unified reservation station_ is one where, instead of having separate reservation stations for different execution units (add, mul, etc.) there is one big one to maximize utilization. However, the dispatch and broadcast logic complexity increases.

A _superscalar processor_ is one that simultaneously fetches, decodes, issues, dispatches, broadcasts, and commits more than one instruction per cycle each.

CPU manufacturer terminology differs from issue, dispatch, and commit:

* issue: issue, allocate, dispatch
* dispatch: dispatch, execute, issue
* commit: commit, complete, retire, graduate

The instruction execution process is:

1. fetch
2. decode
3. issue
4. execute
5. write result (broadcast)
6. commit

Steps 1-3 (fetch, decode, issue) are done in-order to ensure that discovered dependencies respect the original program order. Steps 4-5 (execute, broadcast) are done out-of-order to enable execution in the order of the data dependencies. Step 6 (commit) is done in-order to give the appearance that instructions are executed in-order.

In a Tomasulo-like scheduling policy, memory writes occur at the commit phase, to ensure that the instruction really will be executed.

## Load-Store Queue

A _Load-Store Queue_ (LSQ) is used to supply values from previous stores to future loads. Each entry contains a load/store bit, memory address, value to be stored in memory or value that was loaded from memory, and a completion bit. Load and store instructions get LSQ entries instead of a reservation station.

When an load entry is added to a Load-Store Queue, the LSQ is checked to see if any previous store entry in the queue matches that address. If so, a store-to-load forwarding is performed to copy the value from that entry without ever having gone to memory.

If a new load entry causes a check of previous store entries for an address match, but one of those store entries hasn't computed its address yet, then the load goes ahead as if no store entry matched, i.e. it access memory. If the store entry's address is eventually computed to be the same as the future load, the load will have read a stale value, since it was meant to have read the data written by the store. For this reason, when stores compute their addresses they check subsequent loads to see if they match the address, in which case they are re-triggered.

As soon as a load instruction receives data, it is broadcast to dependent instructions in reservation stations.

Data from a store instruction is sent to memory during the commit phase.

A load/store instruction is committed by freeing the ROB and LSQ entries for it. For stores, the data is also send to memory.

Load/store instructions are executed in address computation and value production, and a store instruction in particular can do those in any order.

_Tree height reduction_ is a way for the compiler to help with ILP by exploiting associativity in order to break up larger computations into smaller independent computations, thereby reducing overall dependencies, hence reducing the dependency tree height.

``` nasm
;; input
r8 = r2 + r3 + r4 + r5
```

``` nasm
;; before
add r8, r2, r3
add r8, r8, r4
add r8, r8, r5
r8 = (r2 + r3) + (r4 + r5)
```

``` nasm
;; after
add r8, r2, r3
add r7, r4, r5
add r8, r8, r7
```

_Instruction scheduling_ by the compiler aims to reduce stalls by reordering (usually independent) instructions to maximize ILP.

_Loop unrolling_ increases the benefits of instruction scheduling because it increases the window/context of the loop contents, which gives access to more instructions that can be reordered via instruction scheduling.

Loop unrolling decreases the number of instructions that are executed, essentially minimizing the branching overhead.

_Loop unrolling once_ refers to modifying a loop so that it does _one_ extra iteration's-worth of work per iteration, i.e. 2x the normal amount of work. Unrolling twice would mean that that the loop does _two_ extra iteration's-worth of work per iteration, i.e. 3x the normal amount of work.

``` cpp
for (size_t i = 1000; i != 0; i--)
  a[i] = a[i] + s;

// unrolled once
for (size_t i = 1000; i != 0; i = i - 2) {
  a[i]     = a[i]     + s;
  a[i - 1] = a[i - 1] + s;
}
```

_Function call inlining_ eliminates call/return overhead and allows for better instruction scheduling because the function's instructions are inline.

Both loop unrolling and function call inlining increase code size.

A _Very Long Instruction Word_ (VLIW) processor is one that executes one large instruction which does the same work as $N$ "normal" instructions. Examples include Itanium and DSP processors. The advantages are that the compiler does the hard work (via instruction scheduling), so the hardware can be simpler and more energy efficient because there is less to do per instruction (no need to detect dependencies), and they work well on loops and "regular" code. The disadvantages are that the latencies of instructions aren't always the same, and the compiler can't predict them (e.g. on a cache miss), most applications are irregular with lots of decisions and branches, and finally there is a large increase in code size due to no-op insertion due to dependencies.

# Caches

The _locality principle_ states that things that will happen soon are likely to be close to things that just happened. _Temporal locality_ means that if a memory address was accessed recently, it's likely to be accessed again soon. _Spatial locality_ means that if a memory address was accessed recently, an address close to it is likely to be accessed as well.

A cache's _Average Memory Access Time_ (AMAT) is the access time to memory as seen by the processor. It's computed as:

$$ \text {AMAT} = \text {hit time} + \text {miss rate} * \text {miss penalty} $$

A cache's _miss time_ is the overall time it takes to have a cache miss. It's computed as:

$$ \text {miss time} = \text {hit time} + \text {miss penalty} $$

A CPU's Level 1 (L1) cache is the cache that directly services read/write requests from the processor. It's usually 16-64KB in size, large enough to get about 90% hit rate, and small enough for the hit time to be about 1-3 cycles.

A _cache block_ or _line size_ is how many bytes are stored in each cache entry. The cache block size should be at least as large as the largest single memory access, but large enough to exploit spatial locality, so that more data is brought in on each access. Typically block sizes on L1 caches are not much bigger than 32-128 bytes because it's a balance between exploiting spatial locality while minimizing the possibility that space goes wasted when programs don't exhibit spatial locality.

If the cache block size is smaller than the largest single memory access, then each memory access will incur multiple cache look-ups for each single memory access.

If the cache block size is not much larger than the largest single memory access, then spatial locality will not be exploited because the local data will not fit in the entry.

If the cache block size is very large but the program does not exhibit much spatial locality, then the space is wasted since the data is never used.

In caches, blocks start at block-aligned addresses. For example, for 64 B blocks, block 1 would be 0-63, block 2 would be 64-127, and so on.

On a given memory access, the region of memory that is brought into the cache is the region that contains the data and is block-aligned, that is, the same size as the cache block.

A _cache line_ is a slot into which a cache block fits. To differentiate, the cache block is the actual data fetched from memory which is inserted into a slot called a cache line.

The _block number_ is the block that corresponds to the memory address that is being accessed. It's determined by dividing the memory address by the block size.

$$ \text {block number} = \frac {\text {address}} {\text {block size}} $$

The _block offset_ is the offset into a particular block that corresponds to a particular memory address. This is necessary because each block may contain more than one memory address' data. It's computed as the remainder of the memory address divided by the block size.

$$ \text {block offset} = {\text {address }} \bmod {\text { block size}} $$

The number of bits used in a cache block offset is determined by the size capacity in bytes (if byte-addressable) of a cache line, or $\log_2 (\text {cache line byte size})$.

The block size should be a power of 2 because it simplifies the process of determining the appropriate block for a particular memory address, which is done by dividing by the block size. Dividing by $2^k$ is just a right-shift by $k$ bits, i.e. discarding the lower $k$ bits determines the block number. This is much simpler and faster than dividing by a non-power of 2.

<img src="https://i.imgur.com/jX1dCZp.png" class="center" />

For example, given a block size of $2^4$ = 16 bytes and a 32-bit address, the block offset is the least significant (lower) 4 bits and the block number is the most significant 28 bits.

``` cpp
// address / 2^4
block_number = address >> 4;

// address % 2^4
block_offset = address & 0xFF; // i.e. 0b1111
```

For a cache to determine if a block is actually in the cache, it keeps a tag for each cache block. The tag of a given address usually consists of at least one bit from the block number component---even the entire block number can be the tag. On a given memory access, the cache checks to see if the tag for that address is present in the cache, in which case the corresponding cache block contains the data.

In order to prevent garbage data from being used after booting a CPU, the cache keeps a valid bit for each cache line which specifies whether the tag and data are valid. Once data is read from memory, the valid bit is enabled. This way, a hit is successful if the address' tag is present _and_ the valid bit is on.

A cache size is typically expressed as how much data it can contain, but it is actually larger due to cache book-keeping overhead such as storing the tag, valid and dirty bits, LRU counters, etc.

A _multi-level cache_ (cache hierarchy) can help to reduce the miss penalty because additional cache levels are still much faster than accessing memory.

The _Last Level Cache_ (LLC) is the cache with no more caches beneath it, so that misses go directly to memory. For example, in a single-level cache, the L1 cache is the LLC.

A cache's _local hit rate_ is the hit rate that the cache actually observes for accesses to this particular cache, dependent on the hit rate of the caches above it.

$$ \text {local hit rate} = \frac {\text {number of hits}} {\text {number of all accesses to this cache}} $$

A cache's _local miss rate_ is simply the complement of the local hit rate:

$$ \text {local miss rate} = 1 - \text {local hit rate} $$

A cache's _global miss rate_ is the number of misses in this cache divided by all memory accesses that the CPU makes:

$$ \text {global miss rate} = \frac {\text {number of misses in this cache}} {\text {number of all memory accesses made by CPU}} $$

A cache's _global hit rate_ is simply the complement of the global miss rate:

$$ \text {global hit rate} = 1 - \text {global miss rate} $$

_Misses per 1000 Instructions_ (MPKI) is a metric of how often the cache hits, which tries to capture the behavior of non-L1 caches:

$$ \text {MPKI} = \frac {\text {number of misses in this cache}} {1000} $$

_Cache inclusion_ means that if a block is in the L1 cache, then it also has to be in the L2 cache.

_Cache exclusion_ means that if a block is in the L1 cache, then it cannot also be in the L2 cache.

If cache inclusion nor cache exclusion is enforced, then if a block is in the L1 cache, it may or may not be in the L2 cache. To maintain the _inclusion property_, each cache line in L2 needs an _inclusion bit_ which is set if the block is also in L1, so that blocks in L2 that are also in L1 are never replaced.

The benefits of enforcing the inclusion property are that it ensures that an L1 write-back is an L2 hit, or that an L1 write-through actually happens in L2 and not in memory, and it also speeds up coherence since only the L2 needs to be probed: if it's not in L2, it won't be in L1, so the L1 doesn't need to be probed.

## Non-Blocking Caches

A _non-blocking cache_ is one that doesn't block until each operation is finished, unlike a _blocking cache_.

In a non-blocking cache, a _hit-under-miss_ is when the cache can continue to serve the cache hits while the cache is waiting on a miss.

In a non-blocking cache, a _miss-under-miss_ is when multiple requests to memory can be made even while a cache miss is already underway. This is an example of _memory-level parallelism_ and requires memory hardware support.

For a cache to support miss-under-miss functionality, it requires _Miss Status Handling Registers_ (MSHRs), which remember what was requested from memory (i.e. information about ongoing misses).

On a miss, the MSHRs are checked for a match to determine if it's a new or existing miss. If it's a new miss, an MSHR is allocated and it remembers which instruction in the processor to wake up. This is called a _miss_.

If it's an existing miss which is under way, and the data hasn't come back yet, the instruction is added to the existing MSHR for that miss. This is called a _half-miss_.

When the data finally comes back from memory, all instructions in the MSHR are woken up, then the MSHR is released.

The more MSHRs there are, the more memory-level parallelism that can be leveraged because multiple misses can be serviced at the same time.

## Direct-Mapped Cache

A _direct-mapped cache_ is one where, for a given cache block, there is exactly one cache line where it may go.

A direct-mapped cache can be thought of as a special instance of a set-associative cache where there is a set for each cache line, i.e. a 1-way set associative cache. This way the index bits are still used to determine the cache line.

In a direct-mapped cache, a cache block's cache line is determined by using some _index bits_, which are taken from above the block offset component. The number of index bits used is determined by the number of lines in the cache, in particular, $\log_2 (\text {total lines})$. The tag comprises the rest of the bits, essentially identifying which of all possible blocks that can go in that cache line is actually present.

<img src="https://i.imgur.com/j3Q5olI.png" class="center" />

The advantage of a direct-mapped cache is that there is only one place to look-up for a block, making it fast, cheap, and energy-efficient. However, since a given block can only go in _one_ place, it increases contention/conflicts for each cache line, which increases the miss rate.

The cache block offset granularity is in bytes (on x86), or the smallest addressable unit of memory.

## Set-Associative Cache

A _set-associative cache_ is one where, for a given cache block, there are $N$ cache lines where it may go. Cache lines are grouped into sets. The number of sets there are is equal to the number of cache lines divided by $N$. A cache is _n-way set-associative_ when a particular block can be in one of $N$ lines within a unique set. That is, each block maps to a unique set, but may choose among the lines within that set.

The number of index bits used in an n-way set-associative cache is $\log_2 (\text {number of sets})$.

The associativity of a cache refers to how many cache lines are assigned to each set. More cache lines per set means the cache is more associative.

## Fully-Associative Cache

A _fully associative cache_ is one where any cache block can be in any cache line, so each tag must be checked in order to see if a block is present.

A fully associative cache can be thought of as a special instance of a set-associative cache where there is one set containing all of the cache lines. This means that there are no index bits, because $\log_2(\text {number of sets = 1}) = 0$.

An address in a fully associative cache doesn't require an index component because the tag alone identifies the correct line, since any block can go in any line in a fully associative cache.

## Cache Replacement

Cache line replacement is necessary when a set is full or there was a cache miss and we need to put a new block in the set.

Possible cache replacement policies include random, Round-Robin (FIFO), and Least Recently Used (LRU).

### Least Recently Used

LRU is a good policy because it exploits temporal locality. It works by maintaining an LRU counter that is set to a different count for each cache line in the set. A count of 0 is considered to be the Least Recently Used.

When a block needs to be replaced, the least recently used block of count 0 is replaced, its cache line's LRU counter is set to the max (i.e. marked MRU), and all other LRU counters are decremented.

When an LRU block is re-accessed, its LRU counter is set to the max (i.e. marked MRU) and all other LRU counters are decremented.

When the MRU block is re-accessed, there is no change.

When a block that's not the LRU or MRU is accessed, its LRU counter is set to the max (i.e. marked MRU) and all other LRU counters with a count _greater_ than the previous count of this block are decremented.

For an n-way set-associative cache, an LRU implementation requires $N$ counters (one for each cache line) of bit size $\log_2 (n)$, for each set.

A counter-based LRU cache implementation requires all $N$ counters in the set to be updated on each access, _even on cache hits_.

### Not Most Recently Used

Not Most Recently Used (NMRU) is an approximation of LRU which works by tracking which block has been used most recently, then picking a random block aside from that one, so that the just-used block isn't replaced. This entails keeping one MRU pointer per set. For example, in a 2-way set-associative cache, there would be a 1-bit pointer per set to specify which of the two entries in the set is the MRU. In general, a pointer of size $\log_2 (N)$ is required per set.

NMRU is much cheaper than true LRU because each set only requires one pointer to the MRU, compared to true LRU where each entry in each set requires its own counter.

The main disadvantage of NMRU is that it only keeps track of the MRU and chooses any of the other blocks for replacement, which may not be the actual LRU.

### Pseudo-Least Recently Used

The Pseudo-LRU (PLRU) replacement policy works by keeping one bit per line in each set. All bits are set to 0, and they are then set to 1 whenever their block is accessed. On replacement, any of the blocks with a 0 bit is eligible for replacement. This essentially means that all of the recently accessed blocks will have a 1 bit.

When the final 0 bit block is replaced and its bit is set to 1, all of the other block's bits (which should be 1, since this is the last 0 bit block) are set to 0.

The performance of PLRU is between true LRU and NMRU. When there's only one 0 bit block left, it's essentially like LRU. When there's only one 1 bit block, it's essentially NMRU. And when there are more than one but not all 1 bit blocks, it's in between.

## Cache Write Policies

A cache write policy's allocate policy concerns whether a block that was written to should be brought into the cache.

A _write-allocate cache_ is one where, when a memory block is written to, it's also brought into the cache.

A _no-write-allocate cache_ is one where, when a memory block is written to, it's _not_ brought into the cache.

A _write-through cache_ is one where, when a cache block is written to, the memory is updated immediately.

A _write-back cache_ is one where, when a cache block is written to, the memory isn't updated until that cache block is replaced. A dirty bit is used to determine if the block was modified since it was placed in the cache, which means that memory has to be updated with its data.

Most modern processors are write-back in order to exploit write-locality and eliminate the cost of sending to memory many times. Most modern processors are write-allocate in order to exploit locality between reads and writes, that is, if we write to something, we're also likely to read it.

Write-allocate and write-back work together because write-allocate will bring data into memory on a write-miss in order to continue to benefit from write-back (writing only to cache block until it is replaced).

## Cache Misses

The three main causes of cache misses are the "3C's" (three C's): _compulsory misses_, _capacity misses_, and _conflict misses_.

A _compulsory miss_ is a miss incurred when the block is accessed for the first time. It's compulsory because it _has_ to happen in order to bring the block into the cache for the first time. In other words, it would be a miss even if the cache were of infinite size.

A _capacity miss_ is a miss on a block that was previously evicted because of limited cache size.  In other words, it would be a miss even in a fully-associative cache of the same size, that is, even if there had been many lines to choose from.

A _conflict miss_ is one that occurs due to a conflict within a set, because the block was previously replaced due to limited associativity (not enough lines per set). It would _not_ have been a miss in a fully-associative cache of the same size.

## Prefetching

_Prefetching_ refers to predicting which blocks will be accessed soon and bringing them into the cache ahead of time, before they're actually accessed. Prefetching could eliminate cache misses, but mispredictions could lead to cache pollution, which leads to other misses because useful data was replaced.

_Cache pollution_ refers to bringing unused junk data into the cache, potentially replacing data that was actually being used.

_Prefetch instructions_ are instructions which the compiler or programmer can use to request prefetches.

The correct amount of data to prefetch via prefetch instructions is difficult to get right because it's largely dependent on hardware, and it's possible to prefetch too little or too much data.

If the amount of data prefetched is too small, it might arrive long after it's actually needed, negating the purpose of prefetching.

If the amount of data prefetched is too large, the data may be evicted by the time it's actually needed.

Furthermore, if the CPU gets faster but the memory doesn't (as is often the case), the time at which the program needs the data will change, necessitating a change in the amount of data to prefetch.

_Hardware prefetching_ is prefetching done by the hardware (e.g. CPU or cache), which requires no changes to the program.

A _stream buffer prefetcher_ is a hardware prefetcher that is sequential in nature. It tries to predict if some sequence of blocks following the one that was just accessed might be accessed next, in which case it tries to prefetch several blocks in advanced.

A _stride prefetcher_ is a hardware prefetcher that tries to determine if memory accesses are at a fixed distance from each other, in which case it prefetches data at subsequent fixed distances [^stride_prefetcher].

[^stride_prefetcher]: I wonder if this kind of prefetcher is meant for, for example, if someone iterates through a 2D array the "wrong" way, in column-major order, so first the first column of the first row, then the first column of the second row, and so on. Then a good stride prefetcher might recognize this access pattern and prefetch the appropriate column stripes. Though hopefully this would be resolved much sooner by [loop interchange](#loop-interchange).

A _correlating prefetcher_ is a hardware prefetcher that tries to detect patterns of memory access sequences, so that when it detects a repeat of a pattern, it prefetches the remaining sequence up to some number. This is good for linked lists which aren't sequential in memory nor at fixed strides. Traversing the linked list in the same manner another time would yield a prefetch.

# Virtual Memory

Virtual memory is split up into virtual pages because it decreases the size of the page table compared to if each byte in virtual memory mapped to a byte in physical memory.

A virtual address can be split up into two components: the virtual page number and the page offset.

The number of bits used for the virtual page number is dependent on the number of entries in the page table:

$$ \text {virtual page number bitsize} = \log_2 (\text {page table size}) $$

The remaining bits are used for the page offset.

A flat page table is one where there is a page table entry for every page number, that is, 1 entry per page in the entire virtual address space, even those regions unused by the program. Each entry contains the frame number and the access bits. The size of a flat page table is:

$$ \frac {\text {virtual memory}} {\text {page size}} * \text {entry size} $$

The problem with flat page tables is that they take up space even for virtual address space that isn't being used by a process. Since a page table exists for each process, this can become prohibitively expensive for much larger address spaces.

A multi-level page table is able to save a lot of space by adding levels of indirection and only allocating the regions of virtual address space that are actually used.

A virtual address is broken down into the virtual page number and page offset as usual, but the page number is broken down further into outer and inner indices into the multi-level page table.

Larger page sizes lead to smaller page tables but increased internal fragmentation. Smaller page sizes lead to larger page tables.

On each memory access, the processor needs to:

* compute the virtual address
* compute its page number
* compute the physical address of the page table entry, given the current page table address and the virtual page number
* read the page table entry
* compute the physical address
* access the cache (or memory if cache miss)

On a multi-level page table, for each level, the processor needs to:

* compute the physical address of the page table entry, given the current page table address and the virtual page number
* read the page table entry
* compute the physical address

Without a _translation look-aside buffer_ (TLB), virtual-to-physical address translation is expensive because on multi-level page tables, a mere virtual-to-physical address translation incurs multiple memory accesses because the page tables reside in memory. All this before the actual physical address can be read, which itself is yet another memory access.

A TLB is necessary instead of just using the CPU cache for caching translations because the cache is very big and is mostly for data, whereas a single virtual-to-physical address translation covers an entire page-worth of data, so a specialized cache like the TLB can be much smaller. Further, the general CPU cache would cache each intermediate result from a multi-level page table translation, whereas the TLB only needs to store the translation result.

When there's a TLB miss, the actual translation is made manually using the page tables, and the final result is inserted into the TLB.

_Software TLB-miss handling_ is when it is the operating system's responsibility to handle a TLB miss, including performing the translation manually. This also means that the operating system can use any representation for the page table, since it's the only one that is using it.

_Hardware TLB-miss handling_ is when the processor automatically reads page tables and updates the TLB on a miss. This is much faster than software TLB miss handling, and it requires that the page tables be in a form that the hardware expects.

A TLB is typically fully or highly associative. Since it's already very small and thus fast, it's not necessary to make it directly-mapped as that would increase the miss rate.

The TLB size should be large enough to cover more memory than the cache, so that the TLB hits at least as often as the cache. For example, 64-512 entries. If more space is required, making the a single TLB larger would make it slower, so instead a two-level TLB is used, where L1 is small and fast and L2 is slightly slower but much larger, yet still faster than manually performing the translation.

The three main methods of improving cache performance are those that reduce the AMAT: reducing the hit time, miss rate, and miss penalty.

Reducing the hit time of a cache can be accomplished by:

* reducing the cache size (though bad for miss rate)
* reducing the cache associativity (though bad for miss rate)
* overlapping cache hits with each other
* overlapping cache hits with TLB hits
* optimizing lookup for the common case
* maintaining replacement state more quickly

Cache hits can be overlapped/coalesced by pipelining the cache.

Hit time is affected by the TLB hit latency because the TLB has to be accessed before the cache can be accessed, in order to get the physical address used to search the cache. The overall cache hit latency is the TLB hit latency plus the cache hit latency:

$$ \text {overall cache hit latency} = \text {TLB hit latency } + \text { cache hit latency} $$

A _Physically Indexed, Physically Tagged_ (PIPT) cache (aka _Physically Accessed Cache_, aka _Physical Cache_) is one that is accessed using a physical address.

A _Virtually Accessed cache_ is one that is accessed using the virtual address. On a cache miss, the TLB is checked for the physical address to bring that data into the cache. Realistically, the TLB still needs to be checked to get the address' permission bits. The advantage is that the hit time is now just the cache hit time, since there's no need to look-up the TLB first. However, the TLB still has to be accessed to retrieve for example the permission bits, and the cache must be flushed on each context switch since virtual addresses are specific to a single process.

A _Virtually Indexed, Physically Tagged_ cache is one that is indexed by the virtual address' index bits, while the tag bits come from the physical address. The virtual address' tag bits are used to get the frame number from the TLB. The cache block's tag is then compared to the physical address to determine a hit or miss. The cache and TLB look-ups are done in parallel.

The advantages of a virtually indexed, physically tagged cache are that, since the cache and TLB look-ups are done in parallel, and the TLB is usually small and fast, the total latency is usually just the cache hit time. Unlike a virtually indexed, virtually tagged cache, the cache doesn't need to be flushed on each context switch because although the cache is indexed by the virtual address' index bits, a hit is determined by the physical address' tag bits. Aliasing is not a problem if the cache is small enough.

The disadvantage of a virtually indexed, physically tagged cache is that aliasing can be a problem, i.e. when multiple virtual addresses map to the same physical address, but they would each get their own separate entries, so a write to one may not be seen by the others. This requires additional handling which might negate the aforementioned performance gains.

Aliasing can be avoided. Since a virtual address' page offset is the same as the physical address' frame offset---since pages and frames are the same size---if the cache's index bits are taken from the same region as the page offset bits, then aliasing will not occur, because any number of virtual addresses that map to the same physical address will contain the same page offset, after all, it's the virtual page number that differs, not the offset into the page/frame. This means that any aliased virtual addresses will map to the same cache line.

The cache size must be restricted by the number of page offset bits and the cache block offset bits. Specifically, the number of index bits must be:

$$ \text {index bit size} = \text {page offset bits } - \text { cache block offset bits}$$

For example, given a 4 KB page and 32 B cache blocks, there will be a 12-bit page offset since $\log_2 (4096) = 12$ and a 5-bit block offset since $\log_2 (32) = 5$, resulting in a 7-bit index since 12 - 5 = 7, which amounts to $2^7 = 128$ sets.

The only way to increase the cache size while preventing aliasing in a virtually indexed, physically tagged cache, given that the maximum number of index bits are already being used, is to increase the associativity of the cache. For example, going from a 2-way set-associative cache to a 4-way set-associative cache. However, this increases latency since it introduces more blocks that must be checked for a hit.

_Way prediction_ refers to predicting which line is most likely to hit based on the index bits, instead of checking all of the cache lines in the set. If there is a misprediction, then all other lines are checked as usual.

# Loop Interchange

_Loop interchange_ is when a compiler modifies a nested loop in order to better exploit spacial locality. This is only possible if the compiler can prove that the rearrangement is equivalent to the original code.

For example, if a loop traverses a 2D array in column-major order (first element of the first row, first element of the second row, etc.), loop interchange would change it to row-major order.

Before:

``` cpp
for (i = 0; i < 10; i++)
  for (j = 0; j < 5; j++)
    a[j][i] = 0;
```

After:

``` cpp
for (j = 0; j < 5; j++)
  for (i = 0; i < 10; i++)
    a[j][i] = 0;
```

# Memory
