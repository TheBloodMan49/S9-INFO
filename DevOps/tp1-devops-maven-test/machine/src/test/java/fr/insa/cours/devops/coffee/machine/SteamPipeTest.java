package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.machine.component.SteamPipe;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class SteamPipeTest {
    SteamPipe steamPipe;

    @BeforeEach
    public void setup() {
        steamPipe = new SteamPipe();
    }

    @Test
    public void testDefault() {
        Assertions.assertFalse(steamPipe.isOn());
    }

    @Test
    public void testSetOn() {
        steamPipe.setOn();
        Assertions.assertTrue(steamPipe.isOn());
    }

    @Test
    public void testSetOff() {
        steamPipe.setOn();
        steamPipe.setOff();
        Assertions.assertFalse(steamPipe.isOn());
    }
}
