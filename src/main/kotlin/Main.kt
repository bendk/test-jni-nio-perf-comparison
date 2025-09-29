package dev.gobley.test.jninioperfcomparison

import kotlin.math.abs
import kotlin.math.pow
import kotlin.math.sqrt
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.ExperimentalTime
import java.nio.ByteBuffer
import java.nio.ByteOrder
import kotlin.random.Random

fun main() {
    // Warming up
    test(10)
    for (repeatTimes in 1_000_000..10_000_000 step 1_000_000) {
        test(repeatTimes)
    }
}

fun test(repeatTimes: Int) {
    println(":::::::::: Test with repeatTimes = $repeatTimes ::::::::::")
    val aFirst = Random.nextInt()
    val aSecond = Random.nextDouble()
    val bFirst = Random.nextInt()
    val bSecond = Random.nextDouble()

    val groundTruth = aSecond.pow(aFirst) + bSecond.pow(bFirst)
    testUsing("jni", repeatTimes, groundTruth) {
        RustLibrary.testUsingJni(aFirst, aSecond, bFirst, bSecond)
    }
    val buffer = ByteBuffer.allocateDirect(32)
    buffer.order(ByteOrder.LITTLE_ENDIAN)
    testUsing("nio", repeatTimes, groundTruth) {
        RustLibrary.testUsingNio(buffer, aFirst, aSecond, bFirst, bSecond)
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
