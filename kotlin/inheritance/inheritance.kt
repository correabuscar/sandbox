//import kotlin.math.PI
import kotlin.math.pow
import kotlin.math.sqrt

fun main() {
    println("Hello, bug-prone world!")
    val squareCabin = SquareCabin(6, 50.0)
    //val capacity=1 //lul, 'with' has no effect on this
    with (squareCabin) {
    	println("\nSquare Cabin\n============")
    	println("Capacity: ${capacity}") // 1 not 6
    	println("Material: ${buildingMaterial}")
   		println("Has room? ${hasRoom()}")
        println("Floor area: ${floorArea()}")
		println("Floor area: %.2f".format(floorArea()))
    }

    val roundHut = RoundHut(3, 10.0)
	with(roundHut) {
	    println("\nRound Hut\n=========")
    	println("Material: ${buildingMaterial}")
    	println("Capacity: ${capacity}")
    	println("Has room? ${hasRoom()}")
        println("Floor area: ${floorArea()}")
		println("Floor area: %.2f".format(floorArea()))
        println("Has room? ${hasRoom()}")
        getRoom()
        println("Has room? ${hasRoom()}")
        getRoom()
		println("Carpet size: ${calculateMaxCarpetSize()}")

	}
    

     val roundTower = RoundTower(4,15.5)
        with(roundTower) {
            println("\nRound Tower\n==========")
            println("Material: ${buildingMaterial}")
            println("Capacity: ${capacity}")
            println("Has room? ${hasRoom()}")
            println("Floor area: ${floorArea()}")
            println("Floor area: %.2f".format(floorArea()))
            println("Carpet size: ${calculateMaxCarpetSize()}")

        }

}

abstract class Dwelling(private var residents: Int){
	abstract val buildingMaterial: String
	abstract val capacity: Int
	fun hasRoom(): Boolean {
	    return residents < capacity
	}

    abstract fun floorArea(): Double

    fun getRoom() {
    if (capacity > residents) {
        residents++
        println("You got a room!")
    } else {
        println("Sorry, at capacity and no rooms left.")
    }
}

}

class SquareCabin(var residents: Int,
                 val length: Double) : Dwelling(residents)  {
    override val buildingMaterial = "Wood"
    override val capacity = 6
    override fun floorArea(): Double {
		return length.pow(2)
	}
}


open class RoundHut(residents: Int, 
   val radius: Double) : Dwelling(residents) {
    override val buildingMaterial = "Straw"
    override val capacity = 4

    override fun floorArea(): Double {
    	//return PI * radius.pow(2) // import needed
        return kotlin.math.PI * radius.pow(2) // import not needed this way
	}
    fun calculateMaxCarpetSize(): Double {
        val diameter = 2 * radius
        return sqrt(diameter.pow(2) / 2)
    }

}

class RoundTower(residents: Int,
                 radius: Double,
    val floors: Int=2) : RoundHut(residents, radius) {
    override val buildingMaterial = "Stone"
    //override val capacity = 4
    override val capacity = 4 * floors
    
    override fun floorArea(): Double {
        return super.floorArea() * floors
    }

}


