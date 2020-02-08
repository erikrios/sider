import javax.swing.JApplet;
import java.awt.*;

public class Einstein extends JApplet {

    // Draw a quotes from Albert Einstein along some shapes
    public void paint(Graphics hal) {
        hal.drawRect(50, 50, 40, 40);
        hal.drawRect(60, 80, 225, 30);
        hal.drawOval(75, 65, 20, 20);
        hal.drawLine(35, 60, 100, 120);

        hal.drawString("Out of clutter, find simplicity.", 110, 70);
        hal.drawString("-- Albert Einstein", 130, 100);
    }
}
