import scala.io.StdIn.readLine

/*
 *   0:      1:      2:      3:      4:
 *  aaaa    ....    aaaa    aaaa    ....
 * b    c  .    c  .    c  .    c  b    c
 * b    c  .    c  .    c  .    c  b    c
 *  ....    ....    dddd    dddd    dddd
 * e    f  .    f  e    .  .    f  .    f
 * e    f  .    f  e    .  .    f  .    f
 *  gggg    ....    gggg    gggg    ....
 *
 *   5:      6:      7:      8:      9:
 *  aaaa    aaaa    aaaa    aaaa    aaaa
 * b    .  b    .  .    c  b    c  b    c
 * b    .  b    .  .    c  b    c  b    c
 *  dddd    dddd    ....    dddd    dddd
 * .    f  e    f  .    f  e    f  .    f
 * .    f  e    f  .    f  e    f  .    f
 *  gggg    gggg    ....    gggg    gggg
 */

object Main {
  val freqMapping: Map[Int, Char] = Map(
    6 -> 'b',
    4 -> 'e',
    9 -> 'f',
  )

  val segmentsToDigit: Map[String, Int] = Map(
    "abcefg"  -> 0,
    "cf"      -> 1,
    "acdeg"   -> 2,
    "acdfg"   -> 3,
    "bcdf"    -> 4,
    "abdfg"   -> 5,
    "abdefg"  -> 6,
    "acf"     -> 7,
    "abcdefg" -> 8,
    "abcdfg"  -> 9,
  )

  def solveMapping(p: List[String]): Map[Char, Char] = {
    val freqs = p.mkString("").groupBy(identity).view.mapValues(_.length)
    val partialMapping = freqs.map { case (k, v) => (k, freqMapping.get(v)) }.collect { case (k, Some(v)) => (k, v) }

    val one = p.find(_.length == 2).get
    val four = p.find(_.length == 4).get
    val seven = p.find(_.length == 3).get
    val a = (seven.toSet -- one.toSet).head
    val c = freqs.collect { case (k, 8) if k != a => k }.head
    val g = freqs.collect { case (k, 7) if !one.contains(k) && !four.contains(k) && !seven.contains(k) => k }.head
    val d = freqs.collect { case (k, 7) if k != g => k }.head

    (partialMapping ++ Map(a -> 'a', c -> 'c', g -> 'g', d -> 'd')).toMap
  }

  def mapOutput(mapping: Map[Char, Char])(o: String): String = {
    o.map(mapping.apply).sorted
  }

  def main(args: Array[String]): Unit = {
    val input: List[(List[String], List[String])] =
      LazyList
        .continually(Option(readLine()))
        .takeWhile(l => !l.isEmpty)
        .collect { case Some(l) => l }
        .map(line => line.split(" \\| ").toList)
        .collect { case List(s, l) => (s.split(" ").toList, l.split(" ").toList) }
        .toList

    val total = input
      .map { case (p, o) => (solveMapping(p), o) }
      .map { case (m, o) => o.map(mapOutput(m)).map(segmentsToDigit.apply).mkString.toInt}
      .sum

    println(total)
  }
}
