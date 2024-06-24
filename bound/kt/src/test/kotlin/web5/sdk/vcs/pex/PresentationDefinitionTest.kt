package web5.sdk.vcs.pex

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import web5.sdk.vc.pex.*

class PresentationDefinitionTest {

  @Test
  fun `test basic presentation definition`() {
    val inputDescriptor = InputDescriptor(
      id = "test_input",
      name = "Test Input",
      purpose = "For testing",
      constraints = ConstraintsData(
        fields = listOf(
          FieldData(
            id = "field1",
            name = "Field 1",
            path = listOf("$.field1"),
            purpose = "Test field",
            filter = FilterData(
              type = "string",
              pattern = "^[a-zA-Z]+$",
              constValue = null
            ),
            optional = false,
            predicate = Optionality.REQUIRED
          )
        )
      )
    )

    val presentationDefinition = PresentationDefinition(
      id = "test_presentation",
      name = "Test Presentation",
      purpose = "For testing purposes",
      inputDescriptors = listOf(inputDescriptor)
    )

    assertEquals("test_presentation", presentationDefinition.id)
    assertEquals("Test Presentation", presentationDefinition.name)
    assertEquals("For testing purposes", presentationDefinition.purpose)
    assertEquals(1, presentationDefinition.inputDescriptors.size)

    val firstInputDescriptor = presentationDefinition.inputDescriptors[0]
    assertEquals("test_input", firstInputDescriptor.id)
    assertEquals("Test Input", firstInputDescriptor.name)
    assertEquals("For testing", firstInputDescriptor.purpose)

    val field = firstInputDescriptor.constraints.fields[0]
    assertEquals("field1", field.id)
    assertEquals("Field 1", field.name)
    assertEquals(listOf("$.field1"), field.path)
    assertEquals("Test field", field.purpose)
    assertFalse(field.optional ?: true)
    assertEquals(Optionality.REQUIRED, field.predicate)

    val filter = field.filter
    assertNotNull(filter)
    assertEquals("string", filter?.type)
    assertEquals("^[a-zA-Z]+$", filter?.pattern)
    assertNull(filter?.constValue)
  }
}