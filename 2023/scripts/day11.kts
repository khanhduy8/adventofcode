import java.io.BufferedReader
import java.io.File
import java.io.FileReader

try{
    val filename = "input_day9.txt"
    val file = File(filename)
    val bufferedReader = BufferedReader(FileReader(file))

    val lines = bufferedReader.readLines().map{it -> it.split(" ").map{ it.toLong()}}
    var sumOfEndElements = 0L
    println(lines)
    for (line in lines){
        val ranks = findEndingElement(line)
        sumOfEndElements += ranks[0].last()
        println(ranks)
    }
    println(sumOfEndElements)

} catch (e: Exception){
    println(e.printStackTrace())
}