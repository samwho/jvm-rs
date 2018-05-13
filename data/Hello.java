public class Hello {
	private final String message;

	public Hello(String message) {
		this.message = message;
	}

	@Override
	public String toString() {
		return this.message;
	}

	public static void main(String... args) {
		System.out.println(new Hello("Hello, world!"));
	}
}
