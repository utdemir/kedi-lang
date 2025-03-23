# 01 - Property testing

Property testing / random testing / fuzzing is not well defined. However, they all end up being providing synthetic inputs to a function and checking if it holds certain properties. I found it very useful over time. But, to me, the biggest catch is side-effects. As writing mocks for every side-effect tends to complicate the code-base, and cumbersome to write. And if we were to use the integration testing approach (spinning up external dependencies like databases or services), it would be slow and flaky.

But - I think a language ecosystem that every effectful library comes with a pure mock version would make it a lot more feasible.

* https://en.wikipedia.org/wiki/Concolic_testing