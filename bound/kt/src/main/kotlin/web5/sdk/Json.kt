package web5.sdk

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.ObjectWriter
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule

internal const val dateTimeFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSXXX"

// this is intended strictly for serializing data over the FFI, not for external use
internal object Json {
    val jsonMapper: ObjectMapper = jacksonObjectMapper()
        .registerKotlinModule()
        .findAndRegisterModules()
        .setSerializationInclusion(JsonInclude.Include.NON_NULL)
        .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)

    private val objectWriter: ObjectWriter = jsonMapper.writer()

    fun stringify(obj: Any): String {
        return objectWriter.writeValueAsString(obj)
    }
}