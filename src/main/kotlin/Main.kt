package dev.gobley.test.jninioperfcomparison

import kotlin.math.abs
import kotlin.math.pow
import kotlin.math.sqrt
import kotlin.random.Random
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.ExperimentalTime
import java.nio.ByteBuffer
import java.nio.ByteOrder

fun main() {
    RustLibrary.initJni()
    val buffer = ByteBuffer.allocateDirect(1024)
    val buffer2 = RustLibrary.getStackBuffer()
    buffer.order(ByteOrder.LITTLE_ENDIAN)
    buffer2.order(ByteOrder.LITTLE_ENDIAN)
    // Warming up
    buffer2.putLong(0, 100)
    test(10, buffer, buffer2, 100)
    for (repeatTimes in 1_000_000..10_000_000 step 1_000_000) {
        // move buffer2 to a random position, this simulates the "stack pointer" being at different
        // positions
        val pos = Random.nextLong(5, 10) * 8
        buffer2.putLong(0, pos)

        test(repeatTimes, buffer, buffer2, pos.toInt())

    }
}

fun test(repeatTimes: Int, buffer: ByteBuffer, buffer2: ByteBuffer, buffer2Pos: Int) {
    println(":::::::::: Test with repeatTimes = $repeatTimes ::::::::::")
    val struct1 = TheStruct.random()
    val struct2 = TheStruct.random()
    val struct3 = TheStruct.random()
    val struct4 = TheStruct.random()

    val groundTruth = (
        struct1.second.pow(struct1.first) +
        struct2.second.pow(struct2.first) +
        struct3.second.pow(struct3.first) +
        struct4.second.pow(struct4.first)
    )
    testUsing("nio", repeatTimes, groundTruth) {
        RustLibrary.testUsingNio(buffer, struct1, struct2, struct3, struct4)
    }
    testUsing("nio2", repeatTimes, groundTruth) {
        RustLibrary.testUsingNio2(buffer2, struct1, struct2, struct3, struct4, buffer2Pos)
    }
    println()
}

@OptIn(ExperimentalTime::class)
fun testUsing(testFnName: String, repeatTimes: Int, groundTruth: Double, testFn: () -> Double) {
    val elapsedTimeList = Array(repeatTimes) { Duration.ZERO }
    repeat(repeatTimes) {
        val startTime = Clock.System.now()
        val result = testFn()
        val endTime = Clock.System.now()
        if((abs(groundTruth - result) / groundTruth) > 0.00001) {
             throw RuntimeException("groundTruth != result ($groundTruth, $result, ${groundTruth-result})")
        }
        elapsedTimeList[it] = endTime - startTime
    }
    val mean = elapsedTimeList.map { it.inWholeNanoseconds }.average()
    val variance = elapsedTimeList
        .map {
            val seconds = it.inWholeNanoseconds
            val difference = mean - seconds
            difference * difference
        }
        .average()
    val stddev = sqrt(variance)
    println("$testFnName: mean = $mean ns, stddev = $stddev ns")
}
