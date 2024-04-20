import Foundation
import UniFFI

print("hello world")
let keyManager = UniFFI.LocalKeyManager.newInMemory()
print(keyManager)