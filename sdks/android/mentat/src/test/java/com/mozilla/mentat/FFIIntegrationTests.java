package com.mozilla.mentat;

import org.junit.Test;

import static org.junit.Assert.*;

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * @see <a href="http://d.android.com/tools/testing">Testing documentation</a>
 */
public class FFIIntegrationTests {
    @Test
    public void open_in_memory_store_succeeds() {
        Mentat store = new Mentat("");
        assertNotNull(store.rawPointer);
    }

    @Test
    public void open_store_in_location_succeeds() {
        Mentat store = new Mentat("test.db");
        assertNotNull(store.rawPointer);
    }
}