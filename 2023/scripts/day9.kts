import java.io.File
import java.io.BufferedReader
import java.io.FileReader

fun findEndingElement(listOfNumber: List<Long>): MutableList<MutableList<Long>>{
    val rank = mutableListOf<MutableList<Long>>()
    rank.add(listOfNumber.toMutableList())

    var nextRank = listOfNumber

    while(nextRank.any { it != 0L }){
        val currentRank = nextRank
        nextRank = mutableListOf()
        for (i in 1 ..< currentRank.size){
            nextRank.add(currentRank[i] - currentRank[i-1])
        }
        rank.add(nextRank)
    }

    var nextElement = 0L

    for (i in rank.size - 1 downTo 0){
        rank[i].add(nextElement)
        if (i != 0){
            nextElement += rank[i - 1].last()
        }
    }

    return rank
}

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