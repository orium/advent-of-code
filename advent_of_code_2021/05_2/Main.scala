import scala.io.StdIn.readLine
import scala.collection.immutable.Range.Inclusive

object Main {
  def delta(a: Int, b: Int): Int =
    if (a > b) {
      -1
    } else if (a < b) {
      1
    } else {
      0
    }

  def path(coords: ((Int, Int), (Int, Int))): List[(Int, Int)] = {
    val ((sx, sy), (dx, dy)) = coords

    if ((sx, sy) == (dx, dy)) {
      List((sx, sy))
    } else {
      (sx, sy) :: path(((sx + delta(sx, dx), sy + delta(sy, dy)), (dx, dy)))
    }
  }

  def main(args: Array[String]): Unit = {
    val input: List[((Int, Int), (Int, Int))] =
      LazyList
        .continually(Option(readLine()))
        .takeWhile(l => !l.isEmpty)
        .collect { case Some(l) => l }
        .map(line => line.split(" ").toSeq)
        .map { case Seq(s, _, l) => (s.split(",").toSeq, l.split(",").toSeq) }
        .map { case (Seq(sx, sy), Seq(dx, dy)) => ((sx.toInt, sy.toInt), (dx.toInt, dy.toInt)) }
        .toList

    val allCoords: List[(Int, Int)] =
      input
        .flatMap(path)

    val counts: Map[(Int, Int), Int] =
      allCoords
        .groupBy(identity)
        .view.mapValues(_.length)
        .toMap

    val aboveTwo: Int =
      counts
        .filter { case (_, count) => count > 1 }
        .toList
        .length

      println(aboveTwo)
  }
}
