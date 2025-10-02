package dev.gobley.test.jninioperfcomparison

import kotlin.math.pow
import kotlin.math.sqrt
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.ExperimentalTime
import java.nio.ByteBuffer
import java.nio.ByteOrder

fun main() {
    RustLibrary.initJni()
    // Warming up
    test(10)
    for (repeatTimes in 1_000_000..10_000_000 step 1_000_000) {
        test(repeatTimes)
    }
}

fun test(repeatTimes: Int) {
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
    testUsing("jni", repeatTimes, groundTruth) {
        RustLibrary.testUsingJni(struct1, struct2, struct3, struct4)
    }
    val buffer = ByteBuffer.allocateDirect(64)
    buffer.order(ByteOrder.LITTLE_ENDIAN)
    testUsing("nio", repeatTimes, groundTruth) {
        RustLibrary.testUsingNio(buffer, struct1, struct2, struct3, struct4)
    }
    println()
}

@OptIn(ExperimentalTime::class)
fun <R> testUsing(testFnName: String, repeatTimes: Int, groundTruth: R, testFn: () -> R) {
    val elapsedTimeList = Array(repeatTimes) { Duration.ZERO }
    repeat(repeatTimes) {
        val startTime = Clock.System.now()
        val result = testFn()
        val endTime = Clock.System.now()
        assert(groundTruth == result)
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
