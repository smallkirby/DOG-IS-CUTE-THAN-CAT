class Cat {
    private fun Exception.toStr() = "\uD83D\uDC36 IS CUTE THAN \uD83D\uDE3C"
    fun meow() {
        println(Exception("\uD83D\uDC31 IS CUTE THAN \uD83D\uDC36").toStr())
    }
}

fun main() {
    Cat().meow()
}