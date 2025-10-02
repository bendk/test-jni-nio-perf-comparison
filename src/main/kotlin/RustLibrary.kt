package dev.gobley.test.jninioperfcomparison

import java.io.File
import java.nio.Buffer
import java.nio.ByteBuffer

object RustLibrary {
    // This value can be configured in the Gradle script.
    private const val RESOURCE_PREFIX = "jvm"
    private const val LIBRARY_NAME = "jni_nio_perf_comparison"

    init {
        val mappedLibraryName = System.mapLibraryName(LIBRARY_NAME)

        // Extract the library file to a temporary location as in JNA so this works even when packaged as a .jar file.
        val isWindows = System.getProperty("os.name").startsWith("Windows")
        val librarySuffix = ".dll".takeIf { isWindows }
        val libraryFile = File.createTempFile(LIBRARY_NAME, librarySuffix)

        RustLibrary::class.java.classLoader!!
            .getResourceAsStream("$RESOURCE_PREFIX/$mappedLibraryName")!!
            .use { inputStream ->
                libraryFile.outputStream().use { outputStream ->
                    inputStream.copyTo(outputStream)
                }
            }

        @Suppress("UnsafeDynamicallyLoadedCode")
        Runtime.getRuntime().load(libraryFile.absolutePath)
    }

    @JvmStatic
    external fun initJni()

    @JvmStatic
    external fun testUsingJni(struct1: TheStruct, struct2: TheStruct, struct3: TheStruct, struct4: TheStruct): Double

    @JvmStatic
    fun testUsingNio(buffer: ByteBuffer, struct1: TheStruct, struct2: TheStruct, struct3: TheStruct, struct4: TheStruct): Double {
        buffer.clear()
        buffer.putInt(0, struct1.first)
        buffer.putDouble(8, struct1.second)
        buffer.putInt(16, struct2.first)
        buffer.putDouble(24, struct2.second)
        buffer.putInt(32, struct3.first)
        buffer.putDouble(40, struct3.second)
        buffer.putInt(48, struct4.first)
        buffer.putDouble(56, struct4.second)
        return testUsingNio(buffer)
    }

    @JvmStatic
    private external fun testUsingNio(structs: Buffer): Double
}
