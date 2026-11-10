
from PySide6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout,
    QHBoxLayout, QLabel, QPushButton,
    QFrame
)
from PySide6.QtCore import QObject, Signal, Qt

from .party_view import PartyView
from core.memory_scanner import MemoryScanner


class ScannerSignals(QObject):
    party_updated = Signal(list)
    scanner_error = Signal(str)


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Emerald IV Scanner")
        self.resize(1150, 760)
        self.setMinimumSize(900, 650)

        self.scanner = MemoryScanner(interval=0.5)

        self.signals = ScannerSignals()
        self.signals.party_updated.connect(self.update_party)
        self.signals.scanner_error.connect(self.show_scanner_error)

        self.connected = False
        self.scanning = False

        self.setup_ui()
        self.apply_styles()

    def setup_ui(self):
        central = QWidget()
        self.setCentralWidget(central)

        main_layout = QVBoxLayout(central)
        main_layout.setContentsMargins(16, 14, 16, 12)
        main_layout.setSpacing(12)

        # Compact header and scanner controls
        toolbar = QFrame()
        toolbar.setObjectName("toolbar")

        toolbar_layout = QHBoxLayout(toolbar)
        toolbar_layout.setContentsMargins(14, 10, 14, 10)
        toolbar_layout.setSpacing(10)

        title_layout = QVBoxLayout()
        title_layout.setSpacing(1)

        title = QLabel("EMERALD IV SCANNER")
        title.setObjectName("mainTitle")

        subtitle = QLabel("POKÉMON EMERALD / HOENN")
        subtitle.setObjectName("subtitle")

        title_layout.addWidget(title)
        title_layout.addWidget(subtitle)

        toolbar_layout.addLayout(title_layout)
        toolbar_layout.addStretch()

        # Connection indicator
        self.connection_dot = QLabel("●")
        self.connection_dot.setObjectName("connectionDot")

        self.connection_status = QLabel("DISCONNECTED")
        self.connection_status.setObjectName("connectionStatus")

        toolbar_layout.addWidget(self.connection_dot)
        toolbar_layout.addWidget(self.connection_status)

        # Scanner buttons
        self.connect_button = QPushButton("CONNECT TO VBA")
        self.connect_button.setObjectName("connectButton")

        self.start_button = QPushButton("START SCANNING")
        self.start_button.setObjectName("startButton")
        self.start_button.setEnabled(False)

        self.stop_button = QPushButton("STOP")
        self.stop_button.setObjectName("stopButton")
        self.stop_button.setEnabled(False)

        self.connect_button.clicked.connect(self.connect_vba)
        self.start_button.clicked.connect(self.start_scanning)
        self.stop_button.clicked.connect(self.stop_scanning)

        toolbar_layout.addWidget(self.connect_button)
        toolbar_layout.addWidget(self.start_button)
        toolbar_layout.addWidget(self.stop_button)

        main_layout.addWidget(toolbar)

        # Scanner status
        status_panel = QFrame()
        status_panel.setObjectName("statusPanel")

        status_layout = QHBoxLayout(status_panel)
        status_layout.setContentsMargins(12, 7, 12, 7)

        self.scan_status = QLabel("Scanner idle")
        self.scan_status.setObjectName("scanStatus")

        status_layout.addWidget(self.scan_status)
        status_layout.addStretch()

        self.scan_mode_label = QLabel("PARTY MONITOR")
        self.scan_mode_label.setObjectName("scanMode")

        status_layout.addWidget(self.scan_mode_label)

        main_layout.addWidget(status_panel)

        # Main two-panel area
        panels = QHBoxLayout()
        panels.setSpacing(12)

        # Your Pokémon
        player_panel = QFrame()
        player_panel.setObjectName("mainPanel")

        player_layout = QVBoxLayout(player_panel)
        player_layout.setContentsMargins(12, 12, 12, 12)
        player_layout.setSpacing(6)

        player_title = QLabel("YOUR POKÉMON")
        player_title.setObjectName("sectionTitle")

        player_description = QLabel("Currently displaying Party Slot 1")
        player_description.setObjectName("panelDescription")

        player_layout.addWidget(player_title)
        player_layout.addWidget(player_description)

        self.party_view = PartyView()
        player_layout.addWidget(self.party_view, 1)

        # Wild Pokémon placeholder
        wild_panel = QFrame()
        wild_panel.setObjectName("mainPanel")

        wild_layout = QVBoxLayout(wild_panel)
        wild_layout.setContentsMargins(14, 12, 14, 12)
        wild_layout.setSpacing(6)

        wild_title = QLabel("WILD POKÉMON")
        wild_title.setObjectName("sectionTitle")

        wild_description = QLabel("Encounter detection will be added later")
        wild_description.setObjectName("panelDescription")

        wild_layout.addWidget(wild_title)
        wild_layout.addWidget(wild_description)

        self.wild_placeholder = QLabel(
            "WAITING FOR ENCOUNTER\n\n"
            "Wild Pokémon detection\nis not active yet."
        )
        self.wild_placeholder.setObjectName("wildPlaceholder")
        self.wild_placeholder.setAlignment(Qt.AlignmentFlag.AlignCenter)

        wild_layout.addWidget(self.wild_placeholder, 1)

        panels.addWidget(player_panel, 1)
        panels.addWidget(wild_panel, 1)

        main_layout.addLayout(panels, 1)

        # Small footer
        footer = QLabel("VISUALBOYADVANCE MEMORY MONITOR")
        footer.setObjectName("footer")
        footer.setAlignment(Qt.AlignmentFlag.AlignCenter)

        main_layout.addWidget(footer)

    def set_connection_status(self, connected):
        self.connected = connected

        self.connection_dot.setProperty(
            "connected", connected
        )

        self.connection_status.setText(
            "VBA CONNECTED" if connected else "DISCONNECTED"
        )

        self.connection_dot.style().unpolish(
            self.connection_dot
        )
        self.connection_dot.style().polish(
            self.connection_dot
        )

    def connect_vba(self):
        try:
            self.scanner.connect()

            self.set_connection_status(True)

            self.connect_button.setEnabled(False)
            self.start_button.setEnabled(True)

            self.scan_status.setText("Connected. Ready to scan.")

        except Exception as error:
            self.set_connection_status(False)
            self.scan_status.setText(
                f"Connection failed: {error}"
            )

    def start_scanning(self):
        if not self.connected or self.scanning:
            return

        self.scanner.start(
            on_update=self.signals.party_updated.emit,
            on_error=lambda error:
                self.signals.scanner_error.emit(str(error))
        )

        self.scanning = True

        self.start_button.setEnabled(False)
        self.stop_button.setEnabled(True)

        self.scan_status.setText(
            "Monitoring Party Slot 1..."
        )

    def stop_scanning(self):
        if not self.scanning:
            return

        self.scanner.stop()

        self.scanning = False

        self.start_button.setEnabled(True)
        self.stop_button.setEnabled(False)

        self.scan_status.setText("Scanning stopped")

    def update_party(self, pokemon_list):
        self.party_view.update_party(pokemon_list)

    def show_scanner_error(self, message):
        self.scan_status.setText(
            f"Scanner error: {message}"
        )

    def apply_styles(self):
        self.setStyleSheet("""
            QMainWindow {
                background-color: #102820;
            }

            QWidget {
                color: #F1F8F2;
                font-family: "Segoe UI";
                font-size: 9pt;
            }

            QFrame#toolbar {
                background-color: #1B4634;
                border: 1px solid #538364;
                border-radius: 10px;
            }

            QLabel#mainTitle {
                color: #E0F5E3;
                font-size: 16pt;
                font-weight: bold;
            }

            QLabel#subtitle {
                color: #A9CBB2;
                font-size: 8pt;
            }

            QLabel#connectionDot {
                color: #D94B4B;
                font-size: 16pt;
            }

            QLabel#connectionDot[connected="true"] {
                color: #63D887;
            }

            QLabel#connectionStatus {
                color: #F1F8F2;
                font-size: 8pt;
                font-weight: bold;
            }

            QPushButton {
                background-color: #398452;
                color: #FFFFFF;
                border: 1px solid #75B88A;
                border-radius: 6px;
                padding: 9px 12px;
                font-size: 8pt;
                font-weight: bold;
            }

            QPushButton:hover {
                background-color: #4B9B64;
            }

            QPushButton:disabled {
                background-color: #405B4A;
                border-color: #536B59;
                color: #AAB8AD;
            }

            QPushButton#stopButton {
                background-color: #923F3F;
                border-color: #C66A6A;
            }

            QPushButton#stopButton:hover {
                background-color: #B84C4C;
            }

            QFrame#statusPanel {
                background-color: #173D2D;
                border: 1px solid #365F46;
                border-radius: 7px;
            }

            QLabel#scanStatus {
                color: #D5EBDD;
                font-size: 9pt;
            }

            QLabel#scanMode {
                color: #9FC7AA;
                font-size: 8pt;
                font-weight: bold;
            }

            QFrame#mainPanel {
                background-color: #173D2D;
                border: 1px solid #538364;
                border-radius: 11px;
            }

            QLabel#sectionTitle {
                color: #E0F5E3;
                font-size: 14pt;
                font-weight: bold;
            }

            QLabel#panelDescription {
                color: #AFCFBA;
                font-size: 8pt;
            }

            QLabel#wildPlaceholder {
                background-color: #204B37;
                border: 2px dashed #659875;
                border-radius: 10px;
                color: #C9E1CF;
                font-size: 11pt;
                font-weight: bold;
                padding: 15px;
            }

            QLabel#footer {
                color: #789E83;
                font-size: 8pt;
            }

            QFrame#pokemonCard {
                background-color: #204B37;
                border: 1px solid #659875;
                border-radius: 9px;
            }

            QFrame#pokemonCard[shiny="true"] {
                background-color: #5B4B22;
                border: 2px solid #F2D46B;
            }

            QFrame#spriteContainer {
                background-color: #315E43;
                border: 1px solid #659875;
                border-radius: 8px;
            }

            QFrame#pokemonCard[shiny="true"] QFrame#spriteContainer {
                background-color: #76602B;
                border-color: #F2D46B;
            }

            QLabel#slotLabel {
                color: #B7E5C2;
                font-size: 8pt;
                font-weight: bold;
            }

            QLabel#pokemonName {
                color: #FFFFFF;
                font-size: 21pt;
                font-weight: bold;
            }

            QLabel#pokemonInfo {
                color: #D5EBDD;
                font-size: 9pt;
            }

            QLabel#identityLabel {
                color: #E4F1E7;
                font-size: 8pt;
            }

            QLabel#shinyLabel {
                color: #FFE18A;
                font-size: 9pt;
                font-weight: bold;
            }

            QLabel#ivTitle {
                color: #E0F5E3;
                font-size: 10pt;
                font-weight: bold;
            }

            QLabel#statLabel {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QProgressBar {
                background-color: #315D45;
                border: 1px solid #739B7C;
                border-radius: 4px;
                min-height: 17px;
                max-height: 17px;
                text-align: center;
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QProgressBar::chunk {
                background-color: #80D69A;
                border-radius: 3px;
            }

            QFrame#separator {
                background-color: #659875;
                max-height: 1px;
                border: none;
            }
        """)

    def closeEvent(self, event):
        self.scanner.disconnect()
        event.accept()
