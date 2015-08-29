---
title = "Python"
published = "August 27, 2015"
comments = false
---

## Learning From Data

The code for the [homework assignments] for the Cal Tech CS 1156x class offered on edX.org. The class was about Machine Learning, focusing on the theory. My class notes are [also available]. I decided to do the assignments in Python to gain some experience with [NumPy], which is quickly becoming the industry choice for scientific computing. I had experience with R, but I preferred to use a "real" language.

[homework assignments]: http://work.caltech.edu/homeworks.html
[also available]: /notes/machine-learning-theory/
[NumPy]: http://numpy.scipy.org/

I created a simple testing mechanism to automate testing of the experiments that were developed, allowing me to do something like:

``` python
# constructor is Question(question_str, choices, answer)
question8 = Question("8. in sample error",
                     [0, 0.1, 0.3, 0.5, 0.8], 'd')

in_sample_error = experiment()
question8.check(in_sample_error)
```

Which would output something like:

```
8. in sample error
  result:  0.506176
  nearest: d. 0.5
  answer:  d. 0.5
  + CORRECT
```

The homework assignments covered many topics such as the simple perceptron learning algorithm, support vector machines, logistic regression via stochastic gradient descent, regularization, and validation.
