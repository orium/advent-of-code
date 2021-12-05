import scala.io.StdIn.readLine
import scala.collection.immutable.Range.Inclusive

object Main {
  def range(a: Int, b: Int): Range.Inclusive =
    if (a < b) { a to b } else { b to a }

  def main(args: Array[String]): Unit = {
    val input =
      Stream
        .continually(Option(readLine()))
        .takeWhile(l => !l.isEmpty)
        .collect { case Some(l) => l }
        .map(line => line.split(" ").toSeq)
        .map { case Seq(s, _, l) => (s.split(",").toSeq, l.split(",").toSeq) }
        .map { case (Seq(sx, sy), Seq(dx, dy)) => ((sx.toInt, sy.toInt), (dx.toInt, dy.toInt)) }

    val allCoords =
      input
        .flatMap {
          case ((sx, sy), (dx, dy)) if sx == dx => range(sy, dy).map(y => (sx, y))
          case ((sx, sy), (dx, dy)) if sy == dy => range(sx, dx).map(x => (x, sy))
          case _ => Stream.empty
        }

    val counts =
      allCoords
        .groupBy(identity)
        .view.mapValues(_.length)

    val aboveTwo =
      counts
        .filter { case (_, count) => count > 1 }
        .toList
        .length

      println(aboveTwo)
  }
}
