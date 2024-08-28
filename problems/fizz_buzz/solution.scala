object Solution {
  def fizzBuzz(n: Int): List[String] = {

    val a = (1 to n).toList
    val b = a.map(x =>
      if (x % 15 == 0) { "FizzBuzz" }
      else if (x % 5 == 0) { "Buzz" }
      else if (x % 3 == 0) { "Fizz" }
      else x.toString()
    )

    return b
  }
}
