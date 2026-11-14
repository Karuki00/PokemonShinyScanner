from PySide6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout,
    QHBoxLayout, QLabel, QPushButton,
    QFrame, QStackedWidget,
    QComboBox
)
from PySide6.QtCore import (
    QObject, Signal, Qt, QTimer, QSettings,
    QPropertyAnimation, QEasingCurve, QRect,
    QEvent, QUrl
)
from PySide6.QtGui import (
    QFontDatabase, QPainter, QColor,
    QLinearGradient, QRadialGradient, QPixmap,
    QPainterPath
)

from pathlib import Path
import math
import random
from PySide6.QtMultimedia import QMediaPlayer, QAudioOutput

import ctypes
from ctypes import wintypes

from .party_view import PartyView
from .pokemon_card import PokemonCard
from core.memory_scanner import MemoryScanner
from core.pokemon_data import get_pokemon


class ScannerSignals(QObject):
    party_updated = Signal(list)
    wild_updated = Signal(dict)
    scanner_error = Signal(str)


class GlowBackdrop(QFrame):
    """Dark gradient background with soft colour glows.
    The glows sit behind the glass panels so the
    translucency is actually visible."""

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        w = self.width()
        h = self.height()

        base = QLinearGradient(0, 0, 0, h)
        base.setColorAt(0.0, QColor("#07130F"))
        base.setColorAt(1.0, QColor("#030907"))
        painter.fillRect(self.rect(), base)

        def glow(cx, cy, radius, color, alpha):
            gradient = QRadialGradient(cx, cy, radius)
            c = QColor(color)
            c.setAlpha(alpha)
            gradient.setColorAt(0.0, c)
            c.setAlpha(0)
            gradient.setColorAt(1.0, c)
            painter.fillRect(self.rect(), gradient)

        r = max(w, h)
        glow(w * 0.15, h * 0.20, r * 0.55, "#2FD4A0", 70)
        glow(w * 0.85, h * 0.80, r * 0.60, "#2E7BE0", 60)
        glow(w * 0.60, h * 0.15, r * 0.40, "#7AE060", 35)
        glow(w * 0.08, h * 0.72, r * 0.16, "#DDEBFF", 120)
        glow(w * 0.86, h * 0.30, r * 0.19, "#B8D7FF", 105)
        glow(w * 0.62, h * 0.86, r * 0.24, "#C8E4FF", 90)

        painter.end()


class HuntTransitionOverlay(QWidget):
    """Rounded glass slab used for hunt-menu page changes."""

    def __init__(self, parent=None):
        super().__init__(parent)
        self._rendered_overlay = QPixmap()

    def resizeEvent(self, event):
        super().resizeEvent(event)
        self._render_overlay()

    def _render_overlay(self):
        if self.width() <= 0 or self.height() <= 0:
            self._rendered_overlay = QPixmap()
            return

        overlay = QPixmap(self.size())
        overlay.fill(Qt.GlobalColor.transparent)
        painter = QPainter(overlay)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        inset = 5
        panel_rect = self.rect().adjusted(
            inset, inset, -inset, -inset
        )
        radius = min(28, panel_rect.height() // 2)

        panel_path = QPainterPath()
        panel_path.addRoundedRect(
            panel_rect,
            radius,
            radius
        )

        glass = QLinearGradient(
            panel_rect.topLeft(),
            panel_rect.bottomRight()
        )
        glass.setColorAt(0.0, QColor(255, 255, 255, 245))
        glass.setColorAt(0.32, QColor(244, 255, 252, 238))
        glass.setColorAt(0.68, QColor(205, 241, 238, 225))
        glass.setColorAt(1.0, QColor(165, 218, 220, 220))
        painter.fillPath(panel_path, glass)

        highlight = QLinearGradient(
            panel_rect.left(),
            panel_rect.top(),
            panel_rect.right(),
            panel_rect.top()
        )
        highlight.setColorAt(0.0, QColor(255, 255, 255, 0))
        highlight.setColorAt(0.44, QColor(255, 255, 255, 0))
        highlight.setColorAt(0.52, QColor(255, 255, 255, 150))
        highlight.setColorAt(0.60, QColor(255, 255, 255, 0))
        highlight.setColorAt(1.0, QColor(255, 255, 255, 0))
        painter.fillPath(panel_path, highlight)

        painter.setPen(QColor(255, 255, 255, 230))
        painter.drawPath(panel_path)
        painter.setPen(QColor(174, 241, 235, 190))
        painter.drawRoundedRect(
            panel_rect.adjusted(2, 2, -2, -2),
            max(1, radius - 2),
            max(1, radius - 2)
        )
        painter.end()
        self._rendered_overlay = overlay

    def paintEvent(self, event):
        painter = QPainter(self)
        if self._rendered_overlay.isNull():
            self._render_overlay()
        painter.drawPixmap(0, 0, self._rendered_overlay)
        painter.end()


class PcBoxBackdrop(QWidget):
    """Cropped storage-box texture used behind the Pokémon summary."""

    THEMES = (
        "Meadow", "Stone", "Desert", "Field",
        "Lavender", "Volcanic", "Ice", "Cave",
        "Beach", "Ocean", "Tidal", "Sky",
        "Battle", "Metal", "Crimson", "Clean",
        "Legendary", "Shadow", "Poké Ball", "League",
        "Floral", "Dark", "Map", "Contest",
    )

    def __init__(self, parent=None):
        super().__init__(parent)
        assets_dir = Path(__file__).resolve().parent.parent / "assets"
        self.source_path = assets_dir / "pc_box" / "background-pc.png"
        self.theme_index = 0
        self.setAttribute(
            Qt.WidgetAttribute.WA_TransparentForMouseEvents
        )

    def set_theme(self, theme_index):
        self.theme_index = max(
            0,
            min(theme_index, len(self.THEMES) - 1)
        )
        self.update()

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.SmoothPixmapTransform)

        source = QPixmap(str(self.source_path))
        if not source.isNull():
            column = self.theme_index % 4
            row = self.theme_index // 4
            tile_left = 9 + column * 172
            tile_top = 38 + row * 164
            tile = source.copy(
                QRect(
                    tile_left + 6,
                    tile_top + 5,
                    145,
                    105
                )
            )
            scaled = tile.scaled(
                self.size(),
                Qt.AspectRatioMode.KeepAspectRatioByExpanding,
                Qt.TransformationMode.SmoothTransformation
            )
            x = (scaled.width() - self.width()) // 2
            y = (scaled.height() - self.height()) // 2
            painter.drawPixmap(
                0,
                0,
                scaled.copy(QRect(x, y, self.width(), self.height()))
            )

        painter.fillRect(
            self.rect(),
            QColor(3, 12, 14, 158)
        )
        painter.fillRect(
            self.rect().adjusted(1, 1, -1, -1),
            QColor(10, 22, 24, 72)
        )
        painter.setPen(QColor(220, 245, 236, 115))
        painter.drawRoundedRect(
            self.rect().adjusted(1, 1, -2, -2),
            18,
            18
        )
        painter.end()


class FloatingPokemonLayer(QWidget):
    """Cached, non-overlapping decorative sprites for the dashboard."""

    SPRITES = (
        (1, 0.22, 0.72, 22, 0.0),
        (25, 0.50, 0.58, 18, 1.8),
        (133, 0.78, 0.64, 26, 3.4),
    )

    def __init__(self, parent=None):
        super().__init__(parent)
        assets_dir = Path(__file__).resolve().parent.parent / "assets"
        self.sprite_dir = assets_dir / "pokemon"
        self.setAttribute(
            Qt.WidgetAttribute.WA_TransparentForMouseEvents
        )

        self.sprite_labels = []
        self.sprite_data = []
        self.refresh_timer = QTimer(self)
        self.refresh_timer.timeout.connect(self.regenerate_sprites)
        self.elapsed = 0.0
        self.regenerate_sprites()

        self.animation_timer = QTimer(self)
        self.animation_timer.setInterval(40)
        self.animation_timer.timeout.connect(self.animate_sprites)
        self.animation_timer.start()

    def regenerate_sprites(self):
        for label in self.sprite_labels:
            label.deleteLater()

        self.sprite_labels.clear()
        self.sprite_data.clear()

        species_ids = random.sample(
            range(1, 387),
            len(self.SPRITES)
        )
        for index, species_id in enumerate(species_ids):
            _, x, scale, amplitude, phase = (
                random.choice(self.SPRITES)
            )
            zone_x = (0.16, 0.50, 0.84)[index]
            x = max(
                zone_x - 0.035,
                min(zone_x + 0.035, x + random.uniform(-0.035, 0.035))
            )
            y = random.uniform(0.22, 0.78)
            y = max(0.12, min(0.88, y + random.uniform(-0.045, 0.045)))
            shiny = random.randrange(1248) == 0
            source_dir = (
                self.sprite_dir / "shiny"
                if shiny
                else self.sprite_dir
            )
            pixmap = QPixmap(
                str(source_dir / f"{species_id}.png")
            )
            if pixmap.isNull():
                continue

            label = QLabel(self)
            label.setAttribute(
                Qt.WidgetAttribute.WA_TransparentForMouseEvents
            )
            label.setPixmap(
                pixmap.scaled(
                    112,
                    112,
                    Qt.AspectRatioMode.KeepAspectRatio,
                    Qt.TransformationMode.SmoothTransformation
                )
            )
            label.setWindowOpacity(0.32)
            label.show()
            self.sprite_labels.append(label)
            self.sprite_data.append(
                (x, y, scale, amplitude, phase)
            )

        self.elapsed = 0.0
        self.animate_sprites()

    def set_refresh_interval(self, milliseconds):
        self.refresh_timer.stop()
        if milliseconds > 0:
            self.refresh_timer.start(milliseconds)

    def resizeEvent(self, event):
        super().resizeEvent(event)
        self.animate_sprites()

    def animate_sprites(self):
        width = self.width()
        height = self.height()
        if width <= 0 or height <= 0:
            return

        self.elapsed += 0.04
        for label, (x, y, scale, amplitude, phase) in zip(
            self.sprite_labels,
            self.sprite_data
        ):
            drift = math.sin(self.elapsed * scale + phase)
            bob = int(drift * amplitude)
            target_x = int(width * x) - label.width() // 2
            target_y = int(height * y) - label.height() // 2 + bob

            label.move(target_x, target_y)

    def stop(self):
        self.animation_timer.stop()


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Emerald IV Scanner")
        self.resize(1150, 760)
        self.setMinimumSize(900, 650)

        self.scanner = MemoryScanner(interval=0.5)

        # Persistent shiny hunt counters
        self.settings = QSettings(
            "EmeraldIVScanner",
            "EmeraldIVScanner"
        )

        self.starter_resets = int(
            self.settings.value("hunt/starter_resets", 0)
        )

        self.wild_encounters = int(
            self.settings.value("hunt/wild_encounters", 0)
        )
        self.box_theme_index = int(
            self.settings.value("appearance/box_theme", 0)
        )
        self.floating_refresh_index = int(
            self.settings.value("appearance/floating_refresh", 1)
        )
        self.floating_refresh_options = (
            ("30 SEC", 30_000),
            ("2 MIN", 120_000),
            ("5 MIN", 300_000),
        )

        # Track Ctrl + R key state to count once per key press.
        self.ctrl_r_was_down = False

        # Poll keyboard state while the application is running.
        self.keyboard_timer = QTimer(self)
        self.keyboard_timer.setInterval(20)
        self.keyboard_timer.timeout.connect(
            self.check_starter_reset
        )

        self.signals = ScannerSignals()
        self.signals.party_updated.connect(self.update_party)
        self.signals.wild_updated.connect(self.update_wild)
        self.signals.scanner_error.connect(self.show_scanner_error)

        self.connected = False
        self.scanning = False

        # Asset directories
        assets_dir = Path(__file__).resolve().parent.parent / "assets"
        self.sound_effects_dir = assets_dir / "sound_effects"
        self.fonts_dir = assets_dir / "fonts"

        # Game-style font (optional pixel font in assets/fonts)
        self.game_font_family = self.load_game_font()

        self.setup_ui()

        # PC interface sound player
        self.pc_audio_output = QAudioOutput(self)
        self.pc_audio_output.setVolume(0.45)

        self.pc_player = QMediaPlayer(self)
        self.pc_player.setAudioOutput(self.pc_audio_output)

        # Button sound player
        self.button_audio_output = QAudioOutput(self)
        self.button_audio_output.setVolume(0.35)

        self.button_player = QMediaPlayer(self)
        self.button_player.setAudioOutput(
            self.button_audio_output
        )
        self.button_player.mediaStatusChanged.connect(
            self.reset_button_sound_state
        )
        self.button_sound_active = False

        # Play startup sound
        QTimer.singleShot(
            500,
            lambda: self.play_pc_sound("on")
        )

        QTimer.singleShot(
            0,
            lambda: self.transition_backdrop.setGeometry(
                self.page_stack.rect()
            )
        )

        # Party Pokémon selection
        self.party_view.pokemon_selected.connect(
            self.on_pokemon_selected
        )

        # Add click sounds to important buttons.
        for button in self.findChildren(QPushButton):
            button.clicked.connect(self.play_button_sound)

        self.apply_styles()

        self.update_counter_labels()
        self.keyboard_timer.start()

    # ==========================================
    # FONT
    # ==========================================

    def load_game_font(self):
        """
        Load a pixel font if one is placed in assets/fonts
        (.ttf / .otf), e.g. "Pokemon Emerald Pro" or "Pokemon GB".
        Falls back to a chunky monospace font.
        """

        if self.fonts_dir.exists():
            for font_file in sorted(
                list(self.fonts_dir.glob("*.ttf"))
                + list(self.fonts_dir.glob("*.otf"))
            ):
                font_id = QFontDatabase.addApplicationFont(
                    str(font_file)
                )

                if font_id != -1:
                    families = QFontDatabase.applicationFontFamilies(
                        font_id
                    )

                    if families:
                        return f'"{families[0]}"'

        return '"Consolas", "Courier New", monospace'

    # ==========================================
    # SOUND EFFECTS
    # ==========================================

    def play_pc_sound(self, sound_name):
        """Play PC startup or shutdown sound."""

        if getattr(self, "closing", False):
            return

        sound_path = (
            self.sound_effects_dir
            / f"pc-{sound_name}.mp3"
        )

        if not sound_path.exists():
            print(f"[Audio] Missing: {sound_path}")
            return

        self.pc_player.stop()

        self.pc_player.setSource(
            QUrl.fromLocalFile(str(sound_path))
        )

        self.pc_player.play()

        print(f"[Audio] Playing: {sound_path.name}")

    def play_button_sound(self):
        """Play one complete button cue without overlapping repeats."""

        if self.button_sound_active:
            return

        sound_path = (
            self.sound_effects_dir
            / "button.mp3"
        )

        if not sound_path.exists():
            print(f"[Audio] Missing: {sound_path}")
            return

        self.button_player.setSource(
            QUrl.fromLocalFile(str(sound_path))
        )

        self.button_sound_active = True
        self.button_player.play()

    def reset_button_sound_state(self, status):
        if status in (
            QMediaPlayer.MediaStatus.EndOfMedia,
            QMediaPlayer.MediaStatus.InvalidMedia,
        ):
            self.button_sound_active = False

    def on_pokemon_selected(self, pokemon):
        """Handle party Pokémon selection."""

        self.play_button_sound()
        self.open_pokemon_summary(pokemon)

    def eventFilter(self, watched, event):
        if watched == self.page_stack:
            if event.type() == QEvent.Type.Resize:
                if not self.transitioning:
                    self.transition_backdrop.setGeometry(
                        self.page_stack.rect()
                    )

        if watched == self.summary_page:
            if event.type() == QEvent.Type.Resize:
                self.summary_box_backdrop.setGeometry(
                    self.summary_page.rect()
                )

        if watched == self.main_page:
            if event.type() == QEvent.Type.Resize:
                self.main_box_backdrop.setGeometry(
                    self.main_page.rect()
                )
                self.floating_pokemon_layer.setGeometry(
                    self.main_page.rect()
                )

        return super().eventFilter(watched, event)

    # ==========================================
    # UI
    # ==========================================

    def setup_ui(self):
        central = QWidget()
        self.setCentralWidget(central)

        root_layout = QVBoxLayout(central)
        root_layout.setContentsMargins(0, 0, 0, 0)

        self.page_stack = QStackedWidget()
        root_layout.addWidget(self.page_stack)

        self.main_page = QWidget()
        self.page_stack.addWidget(self.main_page)
        self.main_box_backdrop = PcBoxBackdrop(self.main_page)
        self.main_box_backdrop.setGeometry(
            self.main_page.rect()
        )
        self.main_box_backdrop.lower()
        self.main_box_backdrop.set_theme(
            self.box_theme_index
        )
        self.floating_pokemon_layer = FloatingPokemonLayer(
            self.main_page
        )
        self.floating_pokemon_layer.setGeometry(
            self.main_page.rect()
        )

        main_layout = QVBoxLayout(self.main_page)
        main_layout.setContentsMargins(16, 14, 16, 12)
        main_layout.setSpacing(12)

        # Header bar and scanner controls
        toolbar = QFrame()
        toolbar.setObjectName("toolbar")

        toolbar_layout = QHBoxLayout(toolbar)
        toolbar_layout.setContentsMargins(16, 10, 16, 10)
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

        theme_label = QLabel("BOX THEME")
        theme_label.setObjectName("themeLabel")

        self.box_theme_selector = QComboBox()
        self.box_theme_selector.setObjectName("boxThemeSelector")
        self.box_theme_selector.addItems(PcBoxBackdrop.THEMES)
        self.box_theme_selector.setCurrentIndex(
            max(
                0,
                min(
                    self.box_theme_index,
                    len(PcBoxBackdrop.THEMES) - 1
                )
            )
        )
        self.box_theme_selector.currentIndexChanged.connect(
            self.set_box_theme
        )

        toolbar_layout.addWidget(theme_label)
        toolbar_layout.addWidget(self.box_theme_selector)

        floating_label = QLabel("FLOATING REFRESH")
        floating_label.setObjectName("themeLabel")

        self.floating_refresh_selector = QComboBox()
        self.floating_refresh_selector.setObjectName(
            "boxThemeSelector"
        )
        self.floating_refresh_selector.addItems(
            [label for label, _ in self.floating_refresh_options]
        )
        self.floating_refresh_selector.setCurrentIndex(
            max(
                0,
                min(
                    self.floating_refresh_index,
                    len(self.floating_refresh_options) - 1
                )
            )
        )
        self.floating_refresh_selector.currentIndexChanged.connect(
            self.set_floating_refresh
        )
        self.set_floating_refresh(
            self.floating_refresh_selector.currentIndex()
        )
        toolbar_layout.addWidget(floating_label)
        toolbar_layout.addWidget(self.floating_refresh_selector)
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

        # Scanner status (styled as a dialogue box)
        status_panel = QFrame()
        status_panel.setObjectName("statusPanel")

        status_layout = QHBoxLayout(status_panel)
        status_layout.setContentsMargins(14, 8, 14, 8)

        self.scan_status = QLabel("Scanner idle")
        self.scan_status.setObjectName("scanStatus")

        status_layout.addWidget(self.scan_status)
        status_layout.addStretch()

        self.scan_mode_label = QLabel("PARTY MONITOR")
        self.scan_mode_label.setObjectName("scanMode")

        status_layout.addWidget(self.scan_mode_label)

        main_layout.addWidget(status_panel)

        # Main dual-screen area
        panels = QHBoxLayout()
        panels.setSpacing(12)

        # LEFT: Six Pokémon party slots
        player_panel = QFrame()
        player_panel.setObjectName("mainPanel")

        player_layout = QVBoxLayout(player_panel)
        player_layout.setContentsMargins(16, 14, 16, 14)
        player_layout.setSpacing(10)

        player_title = QLabel("YOUR PARTY")
        player_title.setObjectName("sectionTitle")

        player_description = QLabel(
            "Select a Pokémon to access its summary"
        )
        player_description.setObjectName("panelDescription")

        player_layout.addWidget(player_title)
        player_layout.addWidget(player_description)

        self.party_view = PartyView()
        player_layout.addWidget(self.party_view, 1)

        # RIGHT: Hunt navigation and pages
        hunt_panel = QFrame()
        hunt_panel.setObjectName("mainPanel")
        self.hunt_panel = hunt_panel

        hunt_layout = QVBoxLayout(hunt_panel)
        hunt_layout.setContentsMargins(16, 14, 16, 14)
        hunt_layout.setSpacing(10)

        self.hunt_stack = QStackedWidget()

        # Main menu page
        self.hunt_menu_page = QWidget()
        menu_layout = QVBoxLayout(self.hunt_menu_page)
        menu_layout.setContentsMargins(0, 0, 0, 0)
        menu_layout.setSpacing(12)

        menu_title = QLabel("MAIN MENU")
        menu_title.setObjectName("sectionTitle")

        menu_description = QLabel(
            "Select a shiny hunting mode"
        )
        menu_description.setObjectName("panelDescription")

        menu_layout.addWidget(menu_title)
        menu_layout.addWidget(menu_description)

        self.starter_menu_button = QPushButton(
            "▶  STARTER HUNT"
        )
        self.starter_menu_button.setObjectName("huntMenuButton")
        self.starter_menu_button.clicked.connect(
            lambda: self.open_hunt_page("starter")
        )

        self.wild_menu_button = QPushButton(
            "▶  WILD HUNT"
        )
        self.wild_menu_button.setObjectName("huntMenuButton")
        self.wild_menu_button.clicked.connect(
            lambda: self.open_hunt_page("wild")
        )

        menu_layout.addWidget(self.starter_menu_button)
        menu_layout.addWidget(self.wild_menu_button)
        menu_layout.addStretch()

        self.hunt_stack.addWidget(self.hunt_menu_page)

        # Starter hunt page
        self.starter_hunt_page = QWidget()
        starter_page_layout = QVBoxLayout(
            self.starter_hunt_page
        )
        starter_page_layout.setContentsMargins(0, 0, 0, 0)
        starter_page_layout.setSpacing(12)

        starter_back = QPushButton("◀ BACK TO MENU")
        starter_back.setObjectName("backButton")
        starter_back.clicked.connect(self.show_hunt_menu)

        starter_heading = QLabel("STARTER SHINY HUNT")
        starter_heading.setObjectName("sectionTitle")

        starter_hint = QLabel(
            "Soft reset counter · Ctrl + R while VBA is active"
        )
        starter_hint.setObjectName("panelDescription")
        starter_hint.setWordWrap(True)

        self.starter_hunt_counter_label = QLabel("0")
        self.starter_hunt_counter_label.setObjectName(
            "huntCounterValue"
        )
        self.starter_hunt_counter_label.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        starter_counter_caption = QLabel("TOTAL SOFT RESETS")
        starter_counter_caption.setObjectName("counterDescription")
        starter_counter_caption.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        self.starter_hunt_status = QLabel(
            "No shiny detected yet"
        )
        self.starter_hunt_status.setObjectName("panelDescription")
        self.starter_hunt_status.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        starter_reset = QPushButton("RESET STARTER HUNT")
        starter_reset.setObjectName("resetCounterButton")
        starter_reset.clicked.connect(
            self.reset_starter_counter
        )

        starter_page_layout.addWidget(starter_back)
        starter_page_layout.addWidget(starter_heading)
        starter_page_layout.addWidget(starter_hint)
        starter_page_layout.addWidget(
            self.starter_hunt_counter_label
        )
        starter_page_layout.addWidget(starter_counter_caption)
        starter_page_layout.addWidget(self.starter_hunt_status)

        self.starter_hunt_card = PokemonCard()
        self.starter_hunt_card.slot_label.setText(
            "CURRENT STARTER"
        )

        starter_page_layout.addWidget(
            self.starter_hunt_card,
            1
        )

        starter_page_layout.addWidget(starter_reset)

        self.hunt_stack.addWidget(self.starter_hunt_page)

        # Wild hunt page
        self.wild_hunt_page = QWidget()
        wild_page_layout = QVBoxLayout(self.wild_hunt_page)
        wild_page_layout.setContentsMargins(0, 0, 0, 0)
        wild_page_layout.setSpacing(8)

        wild_back = QPushButton("◀ BACK TO MENU")
        wild_back.setObjectName("backButton")
        wild_back.clicked.connect(self.show_hunt_menu)

        wild_heading = QLabel("WILD SHINY HUNT")
        wild_heading.setObjectName("sectionTitle")

        wild_hint = QLabel(
            "Automatically counted for every new wild encounter"
        )
        wild_hint.setObjectName("panelDescription")
        wild_hint.setWordWrap(True)

        self.wild_hunt_counter_label = QLabel("0")
        self.wild_hunt_counter_label.setObjectName(
            "huntCounterCompact"
        )
        self.wild_hunt_counter_label.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        wild_counter_caption = QLabel("TOTAL ENCOUNTERS")
        wild_counter_caption.setObjectName("counterDescription")
        wild_counter_caption.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )

        self.wild_hunt_status = QLabel(
            "Waiting for a wild encounter"
        )
        self.wild_hunt_status.setObjectName("panelDescription")
        self.wild_hunt_status.setAlignment(
            Qt.AlignmentFlag.AlignCenter
        )
        self.wild_hunt_status.setWordWrap(True)

        starter_wild_reset = QPushButton("RESET WILD HUNT")
        starter_wild_reset.setObjectName("resetCounterButton")
        starter_wild_reset.clicked.connect(
            self.reset_wild_counter
        )

        wild_page_layout.addWidget(wild_back)
        wild_page_layout.addWidget(wild_heading)
        wild_page_layout.addWidget(wild_hint)
        wild_page_layout.addWidget(
            self.wild_hunt_counter_label
        )
        wild_page_layout.addWidget(wild_counter_caption)
        wild_page_layout.addWidget(self.wild_hunt_status)

        self.wild_hunt_card = PokemonCard()
        self.wild_hunt_card.slot_label.setText(
            "LATEST WILD ENCOUNTER"
        )

        wild_page_layout.addWidget(
            self.wild_hunt_card,
            1
        )

        wild_page_layout.addWidget(starter_wild_reset)

        self.hunt_stack.addWidget(self.wild_hunt_page)

        self.hunt_transition_overlay = HuntTransitionOverlay(
            hunt_panel
        )
        self.hunt_transition_overlay.hide()
        self.hunt_transition_animation = None
        self.hunt_transitioning = False

        hunt_layout.addWidget(self.hunt_stack)

        panels.addWidget(player_panel, 2)
        panels.addWidget(hunt_panel, 3)

        main_layout.addLayout(panels, 1)

        # Small footer
        footer = QLabel("VISUALBOYADVANCE MEMORY MONITOR")
        footer.setObjectName("footer")
        footer.setAlignment(Qt.AlignmentFlag.AlignCenter)

        main_layout.addWidget(footer)

        # Pokémon summary page
        self.summary_page = QWidget()
        self.summary_box_backdrop = PcBoxBackdrop(
            self.summary_page
        )
        self.summary_box_backdrop.setGeometry(
            self.summary_page.rect()
        )
        self.summary_box_backdrop.lower()

        summary_layout = QVBoxLayout(self.summary_page)
        summary_layout.setContentsMargins(28, 24, 28, 24)
        summary_layout.setSpacing(16)

        summary_header_frame = QFrame()
        summary_header_frame.setObjectName("summaryHeader")

        summary_header = QHBoxLayout(summary_header_frame)
        summary_header.setContentsMargins(16, 8, 16, 8)

        summary_title = QLabel("POKÉMON SUMMARY")
        summary_title.setObjectName("summaryTitle")

        summary_title_layout = QVBoxLayout()
        summary_title_layout.setSpacing(0)
        summary_title_layout.addWidget(summary_title)

        summary_context = QLabel(
            "LIVE MEMORY SNAPSHOT  ·  GEN III DATA"
        )
        summary_context.setObjectName("summaryContext")
        summary_title_layout.addWidget(summary_context)

        summary_header.addLayout(summary_title_layout)
        summary_header.addStretch()

        self.summary_slot_label = QLabel("PARTY SLOT 1")
        self.summary_slot_label.setObjectName("summarySlot")

        summary_header.addWidget(self.summary_slot_label)

        summary_layout.addWidget(summary_header_frame)

        summary_meta = QHBoxLayout()
        summary_meta.setSpacing(10)

        self.summary_species_meta = QLabel(
            "SPECIES  ·  WAITING FOR DATA"
        )
        self.summary_species_meta.setObjectName("summaryMeta")

        self.summary_identity_meta = QLabel(
            "IDENTITY  ·  PID / OTID"
        )
        self.summary_identity_meta.setObjectName("summaryMeta")

        self.summary_iv_meta = QLabel(
            "GENETICS  ·  IV BREAKDOWN"
        )
        self.summary_iv_meta.setObjectName("summaryMeta")

        summary_meta.addWidget(self.summary_species_meta, 1)
        summary_meta.addWidget(self.summary_identity_meta, 1)
        summary_meta.addWidget(self.summary_iv_meta, 1)
        summary_layout.addLayout(summary_meta)

        self.summary_card = PokemonCard()
        self.summary_card.slot_label.setText("POKÉMON SUMMARY")
        self.summary_card.set_showcase_sprite()

        summary_layout.addWidget(self.summary_card, 1)

        summary_footer = QHBoxLayout()
        summary_footer.setSpacing(12)

        summary_footer_label = QLabel(
            "MEMORY MONITOR  /  DATA LOCKED TO CURRENT PARTY SLOT"
        )
        summary_footer_label.setObjectName("summaryFooter")
        summary_footer.addWidget(summary_footer_label)
        summary_footer.addStretch()

        self.summary_back_button = QPushButton(
            "◀ BACK TO PARTY"
        )
        self.summary_back_button.setObjectName("backButton")
        self.summary_back_button.clicked.connect(
            self.back_to_main
        )

        summary_footer.addWidget(self.summary_back_button)
        summary_layout.addLayout(summary_footer)

        self.page_stack.addWidget(self.summary_page)

        # Transparent pages allow the background animation
        # to remain visible behind the interface.
        self.main_page.setAttribute(
            Qt.WidgetAttribute.WA_TranslucentBackground,
            True
        )
        self.summary_page.setAttribute(
            Qt.WidgetAttribute.WA_TranslucentBackground,
            True
        )

        self.main_page.setStyleSheet(
            "background: transparent;"
        )
        self.summary_page.setStyleSheet(
            "background: transparent;"
        )

        self.page_stack.setStyleSheet(
            "background: transparent;"
        )
        self.page_stack.setAttribute(
            Qt.WidgetAttribute.WA_TranslucentBackground,
            True
        )

        # Background layer: stays behind the actual interface.
        self.transition_backdrop = GlowBackdrop(self.page_stack)

        self.transition_backdrop.setGeometry(
            self.page_stack.rect()
        )
        self.transition_backdrop.lower()

        self.page_transition_overlay = HuntTransitionOverlay(
            self.page_stack
        )
        self.page_transition_overlay.hide()
        self.page_transition_animation = None

        self.page_stack.installEventFilter(self)
        self.summary_page.installEventFilter(self)
        self.main_page.installEventFilter(self)

        self.set_box_theme(self.box_theme_index)

        self.transition_animation = None
        self.transitioning = False

        self.page_stack.setCurrentWidget(self.main_page)

    def open_hunt_page(self, hunt_type):
        target_page = {
            "starter": self.starter_hunt_page,
            "wild": self.wild_hunt_page,
        }.get(hunt_type)

        self.transition_hunt_page(target_page)

    def transition_hunt_page(self, target_page):
        """Switch hunt pages with a bright diagonal wipe."""

        if target_page is None or self.hunt_transitioning:
            return

        self.hunt_transitioning = True
        overlay = self.hunt_transition_overlay
        width = self.hunt_panel.width()
        height = self.hunt_panel.height()
        covered = QRect(0, 0, width, height)
        start = QRect(-width, 0, width, height)
        end = QRect(width, 0, width, height)

        overlay.setGeometry(start)
        overlay.show()
        overlay.raise_()

        sweep_in = QPropertyAnimation(
            overlay,
            b"geometry",
            self
        )
        sweep_in.setDuration(380)
        sweep_in.setStartValue(start)
        sweep_in.setEndValue(covered)
        sweep_in.setEasingCurve(QEasingCurve.Type.OutCubic)

        def reveal_page():
            self.hunt_stack.setCurrentWidget(target_page)

            sweep_out = QPropertyAnimation(
                overlay,
                b"geometry",
                self
            )
            sweep_out.setDuration(460)
            sweep_out.setStartValue(covered)
            sweep_out.setEndValue(end)
            sweep_out.setEasingCurve(QEasingCurve.Type.InCubic)
            self.hunt_transition_animation = sweep_out

            def finish():
                overlay.hide()
                self.hunt_transition_animation = None
                self.hunt_transitioning = False

            sweep_out.finished.connect(finish)
            QTimer.singleShot(90, sweep_out.start)

        sweep_in.finished.connect(reveal_page)
        self.hunt_transition_animation = sweep_in
        sweep_in.start()

    def show_hunt_menu(self):
        self.transition_hunt_page(self.hunt_menu_page)

    def set_box_theme(self, theme_index):
        self.box_theme_index = max(
            0,
            min(theme_index, len(PcBoxBackdrop.THEMES) - 1)
        )

        for backdrop_name in (
            "main_box_backdrop",
            "summary_box_backdrop",
        ):
            backdrop = getattr(self, backdrop_name, None)
            if backdrop is not None:
                backdrop.set_theme(self.box_theme_index)

        self.main_box_backdrop.repaint()

        self.settings.setValue(
            "appearance/box_theme",
            self.box_theme_index
        )
        self.settings.sync()

    def set_floating_refresh(self, refresh_index):
        refresh_index = max(
            0,
            min(refresh_index, len(self.floating_refresh_options) - 1)
        )
        self.floating_refresh_index = refresh_index
        _, interval = self.floating_refresh_options[refresh_index]
        self.floating_pokemon_layer.set_refresh_interval(interval)
        self.settings.setValue(
            "appearance/floating_refresh",
            refresh_index
        )
        self.settings.sync()

    def open_pokemon_summary(self, pokemon):
        self.summary_card.set_pokemon(pokemon)

        slot = pokemon.get("party_slot", 1)

        self.summary_slot_label.setText(
            f"PARTY SLOT {slot}"
        )

        self.summary_card.slot_label.setText(
            f"PARTY SLOT {slot}"
        )

        species_data = get_pokemon(pokemon["species"])
        species_name = (
            species_data["name"]
            if species_data
            else f"UNKNOWN #{pokemon['species']}"
        )
        self.summary_species_meta.setText(
            f"SPECIES  ·  {species_name.upper()}"
        )
        self.summary_identity_meta.setText(
            f"IDENTITY  ·  PID {pokemon['pid']}"
        )
        self.summary_iv_meta.setText(
            "GENETICS  ·  IV BREAKDOWN"
        )

        self.transition_to(self.summary_page)

    def back_to_main(self):
        self.transition_to(self.main_page)

    def transition_to(self, target_page):
        """Switch dashboard pages with a visible glass-slab wipe."""

        if self.transitioning or target_page is None:
            return

        self.transitioning = True

        width = self.page_stack.width()
        height = self.page_stack.height()

        covered = QRect(0, 0, width, height)
        start = QRect(-width, 0, width, height)
        end = QRect(width, 0, width, height)
        overlay = self.page_transition_overlay

        overlay.setGeometry(start)
        overlay.show()
        overlay.raise_()

        sweep_in = QPropertyAnimation(
            overlay,
            b"geometry",
            self
        )
        sweep_in.setDuration(380)
        sweep_in.setStartValue(start)
        sweep_in.setEndValue(covered)
        sweep_in.setEasingCurve(QEasingCurve.Type.OutCubic)

        def reveal_target():
            self.summary_box_backdrop.setGeometry(
                self.summary_page.rect()
            )
            self.page_stack.setCurrentWidget(target_page)
            overlay.raise_()
            overlay.update()

            sweep_out = QPropertyAnimation(
                overlay,
                b"geometry",
                self
            )
            sweep_out.setDuration(460)
            sweep_out.setStartValue(covered)
            sweep_out.setEndValue(end)
            sweep_out.setEasingCurve(QEasingCurve.Type.InCubic)
            self.page_transition_animation = sweep_out

            def finish_transition():
                overlay.hide()
                self.page_transition_animation = None
                self.transitioning = False

            sweep_out.finished.connect(finish_transition)
            QTimer.singleShot(110, sweep_out.start)

        sweep_in.finished.connect(reveal_target)
        self.page_transition_animation = sweep_in
        sweep_in.start()

    # ==========================================
    # SCANNER CONTROL
    # ==========================================

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
                self.signals.scanner_error.emit(str(error)),
            on_wild_update=self.signals.wild_updated.emit
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

    # ==========================================
    # COUNTERS
    # ==========================================

    def update_counter_labels(self):
        self.starter_hunt_counter_label.setText(
            f"{self.starter_resets:,}"
        )

        self.wild_hunt_counter_label.setText(
            f"{self.wild_encounters:,}"
        )

    def save_counters(self):
        self.settings.setValue(
            "hunt/starter_resets",
            self.starter_resets
        )

        self.settings.setValue(
            "hunt/wild_encounters",
            self.wild_encounters
        )

        self.settings.sync()

    def increment_starter_counter(self):
        self.starter_resets += 1
        self.update_counter_labels()
        self.save_counters()

    def increment_wild_counter(self):
        self.wild_encounters += 1
        self.update_counter_labels()
        self.save_counters()

    def reset_starter_counter(self):
        self.starter_resets = 0
        self.update_counter_labels()
        self.save_counters()

        self.scan_status.setText(
            "Starter hunt counter reset"
        )

    def reset_wild_counter(self):
        self.wild_encounters = 0
        self.update_counter_labels()
        self.save_counters()

        self.scan_status.setText(
            "Wild hunt counter reset"
        )

    def is_vba_foreground(self):
        """Check whether VisualBoyAdvance is the foreground process."""

        pm = self.scanner.reader.pm

        if pm is None:
            return False

        vba_pid = getattr(pm, "process_id", None)

        if not vba_pid:
            return False

        user32 = ctypes.windll.user32

        user32.GetForegroundWindow.restype = wintypes.HWND

        user32.GetWindowThreadProcessId.argtypes = [
            wintypes.HWND,
            ctypes.POINTER(wintypes.DWORD)
        ]

        user32.GetWindowThreadProcessId.restype = wintypes.DWORD

        foreground_window = user32.GetForegroundWindow()

        if not foreground_window:
            return False

        foreground_pid = wintypes.DWORD()

        user32.GetWindowThreadProcessId(
            foreground_window,
            ctypes.byref(foreground_pid)
        )

        return foreground_pid.value == int(vba_pid)

    def check_starter_reset(self):
        """Count Ctrl + R only when VBA is the foreground application."""

        try:
            user32 = ctypes.windll.user32

            user32.GetAsyncKeyState.argtypes = [ctypes.c_int]
            user32.GetAsyncKeyState.restype = ctypes.c_short

            ctrl_down = bool(
                user32.GetAsyncKeyState(0x11) & 0x8000
            )

            r_down = bool(
                user32.GetAsyncKeyState(0x52) & 0x8000
            )

            combo_down = ctrl_down and r_down

            # Count only the initial key-down, not every polling cycle.
            if (
                combo_down
                and not self.ctrl_r_was_down
                and self.is_vba_foreground()
            ):
                self.increment_starter_counter()

            self.ctrl_r_was_down = combo_down

        except (AttributeError, OSError):
            return

    # ==========================================
    # SCANNER CALLBACKS
    # ==========================================

    # Shiny status text (white, bold).
    SHINY_STYLE = "color: #FFFFFF; font-weight: bold;"

    def update_party(self, pokemon_list):
        self.party_view.update_party(pokemon_list)

        starter = next(
            (
                pokemon for pokemon in pokemon_list
                if pokemon.get("party_slot") == 1
            ),
            None
        )

        if starter:
            self.starter_hunt_card.set_pokemon(starter)
            self.starter_hunt_card.slot_label.setText(
                "CURRENT STARTER"
            )

        if starter and starter["shiny"]:
            self.starter_hunt_status.setText(
                "★ SHINY STARTER FOUND! ★"
            )
            self.starter_hunt_status.setStyleSheet(
                self.SHINY_STYLE
            )
        elif starter:
            species_id = starter["species"]
            species_data = get_pokemon(species_id)
            species_name = (
                species_data["name"]
                if species_data
                else f"Unknown Species #{species_id}"
            )
            self.starter_hunt_status.setText(
                f"Current starter: {species_name}"
            )
            self.starter_hunt_status.setStyleSheet("")

    def update_wild(self, pokemon):
        self.increment_wild_counter()

        self.wild_hunt_card.set_pokemon(pokemon)
        self.wild_hunt_card.slot_label.setText(
            "LATEST WILD ENCOUNTER"
        )

        species_id = pokemon["species"]
        species_data = get_pokemon(species_id)

        species_name = (
            species_data["name"]
            if species_data
            else f"Unknown Species #{species_id}"
        )

        if pokemon["shiny"]:
            message = f"★ SHINY FOUND: {species_name}! ★"
            self.wild_hunt_status.setText(message)
            self.wild_hunt_status.setStyleSheet(
                self.SHINY_STYLE
            )
        else:
            self.wild_hunt_status.setText(
                f"Latest encounter: {species_name}"
            )
            self.wild_hunt_status.setStyleSheet("")

    def show_scanner_error(self, message):
        self.scan_status.setText(
            f"Scanner error: {message}"
        )

    # ==========================================
    # STYLE - POKÉMON EMERALD MENU THEME
    # ==========================================

    def apply_styles(self):
        stylesheet = """
            /* =========================================
               GLOBAL - DARK BASE, WHITE TEXT
            ========================================= */

            QMainWindow {
                background-color: #050F0B;
            }

            QWidget {
                color: #FFFFFF;
                font-family: __FONT__;
                font-size: 9pt;
            }

            QLabel {
                color: #FFFFFF;
                background: transparent;
            }

            QToolTip {
                background-color: #0B1F18;
                color: #FFFFFF;
                border: 1px solid rgba(255, 255, 255, 120);
                padding: 4px;
            }

            /* =========================================
               GLASS: HEADER
            ========================================= */

            QFrame#toolbar {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 rgba(90, 230, 170, 60),
                    stop:1 rgba(40, 150, 110, 28)
                );
                border: 1px solid rgba(255, 255, 255, 70);
                border-top: 1px solid rgba(255, 255, 255, 140);
                border-radius: 16px;
            }

            QLabel#mainTitle {
                color: #FFFFFF;
                font-size: 16pt;
                font-weight: bold;
                letter-spacing: 2px;
            }

            QLabel#subtitle {
                color: #FFFFFF;
                font-size: 8pt;
                letter-spacing: 1px;
            }

            QLabel#connectionDot {
                color: #FF6060;
                font-size: 16pt;
            }

            QLabel#connectionDot[connected="true"] {
                color: #78F5A0;
            }

            QLabel#connectionStatus {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QLabel#themeLabel {
                color: rgba(225, 245, 240, 190);
                font-size: 7pt;
                font-weight: bold;
                letter-spacing: 1px;
            }

            QComboBox#boxThemeSelector {
                background-color: rgba(5, 14, 17, 185);
                color: #FFFFFF;
                border: 2px solid rgba(220, 245, 238, 110);
                border-top: 2px solid rgba(255, 255, 255, 175);
                border-radius: 8px;
                padding: 5px 9px;
                min-width: 90px;
            }

            QComboBox#boxThemeSelector QAbstractItemView {
                background-color: #0B181B;
                color: #FFFFFF;
                border: 2px solid rgba(220, 245, 238, 120);
                selection-background-color: rgba(80, 190, 160, 180);
            }

            /* =========================================
               GLASS: BUTTONS
            ========================================= */

            QPushButton {
                background-color: rgba(18, 31, 37, 235);
                color: #E8F5F2;
                border: 2px solid rgba(166, 218, 210, 150);
                border-top: 2px solid rgba(245, 255, 252, 220);
                border-radius: 10px;
                padding: 9px 12px;
                font-size: 8pt;
                font-weight: bold;
            }

            QPushButton:hover {
                background-color: rgba(35, 91, 87, 245);
                border: 2px solid rgba(205, 255, 242, 240);
                color: #FFFFFF;
            }

            QPushButton:pressed {
                background-color: rgba(17, 61, 64, 250);
                border: 2px solid #D8FFF5;
                color: #FFFFFF;
            }

            QPushButton:disabled {
                background-color: rgba(20, 27, 30, 190);
                border: 2px solid rgba(170, 190, 188, 80);
                color: rgba(220, 235, 232, 125);
            }

            QPushButton#connectButton {
                background-color: #227A58;
                border-color: #8FE8BE;
            }

            QPushButton#startButton {
                background-color: #245D9C;
                border-color: #9CCBFF;
            }

            QPushButton#stopButton {
                background-color: #873B47;
                border-color: #FFADB7;
            }

            QPushButton#connectButton:hover,
            QPushButton#startButton:hover,
            QPushButton#stopButton:hover {
                background-color: #3A9A75;
                border-color: #FFFFFF;
            }

            QPushButton#connectButton:disabled,
            QPushButton#startButton:disabled,
            QPushButton#stopButton:disabled {
                background-color: rgba(255, 255, 255, 10);
                border: 1px solid rgba(255, 255, 255, 40);
                color: rgba(255, 255, 255, 120);
            }

            /* =========================================
               GLASS: STATUS + PANELS
            ========================================= */

            QFrame#statusPanel {
                background-color: rgba(8, 18, 22, 185);
                border: 2px solid rgba(220, 235, 255, 105);
                border-top: 2px solid rgba(245, 250, 255, 170);
                border-radius: 14px;
            }

            QLabel#scanStatus {
                color: #FFFFFF;
                font-size: 9pt;
                font-weight: bold;
            }

            QLabel#scanMode {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QFrame#mainPanel {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 rgba(36, 48, 53, 190),
                    stop:0.18 rgba(17, 28, 32, 205),
                    stop:0.72 rgba(8, 18, 22, 215),
                    stop:1 rgba(5, 12, 15, 225)
                );
                border: 2px solid rgba(220, 235, 255, 115);
                border-top: 2px solid rgba(245, 250, 255, 190);
                border-radius: 18px;
            }

            QFrame#counterPanel {
                background-color: rgba(0, 0, 0, 70);
                border: 1px solid rgba(255, 255, 255, 50);
                border-radius: 12px;
            }

            QLabel#sectionTitle {
                color: #FFFFFF;
                font-size: 15pt;
                font-weight: bold;
            }

            QLabel#panelDescription {
                color: rgba(220, 239, 235, 225);
                font-size: 8pt;
            }

            QLabel#panelStatus {
                color: #FFFFFF;
                font-size: 8pt;
            }

            QLabel#footer {
                color: rgba(210, 235, 230, 205);
                font-size: 8pt;
                letter-spacing: 2px;
            }

            QLabel#wildPlaceholder {
                background-color: rgba(0, 0, 0, 70);
                border: 1px dashed rgba(255, 255, 255, 110);
                border-radius: 12px;
                color: #FFFFFF;
                font-size: 11pt;
                font-weight: bold;
                padding: 15px;
            }

            /* =========================================
               COUNTERS
            ========================================= */

            QLabel#counterTitle {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QLabel#counterValue {
                color: #FFFFFF;
                font-size: 23pt;
                font-weight: bold;
                padding: 3px;
            }

            QLabel#counterDescription {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
                letter-spacing: 1px;
            }

            QLabel#huntCounterValue {
                color: #FFFFFF;
                font-size: 40pt;
                font-weight: bold;
                padding: 3px;
            }

            QLabel#huntCounterCompact {
                color: #FFFFFF;
                font-size: 26pt;
                font-weight: bold;
                padding: 0px;
            }

            QPushButton#resetCounterButton:hover {
                background-color: rgba(255, 90, 90, 130);
                border: 1px solid rgba(255, 190, 190, 230);
            }

            QPushButton#resetCounterButton {
                background-color: #713A45;
                border: 2px solid #E49BA8;
                color: #FFF4F5;
            }

            /* =========================================
               GLASS: PARTY SLOTS
            ========================================= */

            QToolButton#partySlot {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 rgba(130, 185, 255, 85),
                    stop:1 rgba(60, 110, 200, 40)
                );
                border: 1px solid rgba(200, 225, 255, 120);
                border-top: 1px solid rgba(235, 245, 255, 190);
                border-radius: 14px;
                color: #FFFFFF;
                font-size: 9pt;
                font-weight: bold;
                padding: 7px;
            }

            QToolButton#partySlot:hover {
                background-color: rgba(255, 214, 110, 80);
                border: 1px solid rgba(255, 226, 140, 240);
                color: #FFFFFF;
            }

            QToolButton#partySlot:pressed {
                background-color: rgba(255, 170, 60, 120);
                border: 1px solid #FFE08A;
                color: #FFFFFF;
            }

            QToolButton#partySlot:disabled {
                background-color: rgba(255, 255, 255, 8);
                border: 1px dashed rgba(255, 255, 255, 70);
                color: rgba(255, 255, 255, 130);
            }

            /* =========================================
               HUNT MENU
            ========================================= */

            QPushButton#huntMenuButton {
                background-color: #183C49;
                border: 2px solid #67C9D0;
                border-left: 6px solid #4CE0C1;
                border-radius: 14px;
                min-height: 56px;
                font-size: 12pt;
                font-weight: bold;
                text-align: left;
                padding: 10px 18px;
            }

            QPushButton#huntMenuButton:hover {
                background-color: #226675;
                border-color: #D8FFFA;
                border-left-color: #FFE08A;
            }

            QPushButton#backButton {
                border-radius: 10px;
                padding: 8px 14px;
            }

            /* =========================================
               SUMMARY HEADER
            ========================================= */

            QFrame#summaryHeader {
                background-color: #173C47;
                border: 1px solid #D9FFF5;
                border-radius: 14px;
            }

            QLabel#summaryTitle {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 21pt;
                font-weight: bold;
                letter-spacing: 2px;
            }

            QLabel#summaryContext {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 9pt;
                font-weight: bold;
                letter-spacing: 1px;
            }

            QLabel#summarySlot {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 12pt;
                font-weight: bold;
            }

            QLabel#summaryMeta {
                background-color: #173C47;
                border: 1px solid #8FE8D1;
                border-radius: 9px;
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 10pt;
                font-weight: bold;
                letter-spacing: 0.6px;
                padding: 8px 10px;
            }

            QLabel#summaryFooter {
                color: #D9FFF5;
                font-family: "Segoe UI", "Arial";
                font-size: 9pt;
                font-weight: 600;
                letter-spacing: 1px;
            }

            /* =========================================
               GLASS: POKEMON CARD
            ========================================= */

            QFrame#pokemonCard {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 rgba(255, 255, 255, 36),
                    stop:1 rgba(255, 255, 255, 12)
                );
                border: 2px solid rgba(255, 255, 255, 90);
                border-top: 2px solid rgba(255, 255, 255, 180);
                border-radius: 16px;
            }

            QFrame#pokemonCard[shiny="true"] {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 rgba(255, 215, 90, 90),
                    stop:1 rgba(210, 150, 30, 40)
                );
                border: 1px solid rgba(255, 224, 120, 210);
                border-top: 1px solid #FFF0B0;
            }

            QFrame#spriteContainer {
                background-color: rgba(0, 0, 0, 80);
                border: 1px solid rgba(255, 255, 255, 60);
                border-radius: 12px;
            }

            QFrame#pokemonCard[shiny="true"] QFrame#spriteContainer {
                background-color: rgba(60, 40, 0, 90);
                border-color: rgba(255, 224, 120, 190);
            }

            QFrame#pokemonCard[showcase="true"] QFrame#spriteContainer {
                background-color: transparent;
                border: none;
            }

            QFrame#pokemonCard[showcase="true"] {
                background-color: #102A34;
                border: 2px solid #D9FFF5;
                border-radius: 18px;
                font-family: "Segoe UI", "Arial";
            }

            QFrame#pokemonCard[showcase="true"] QLabel#spriteLabel {
                background: transparent;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#summarySpritePanel {
                background-color: #245D67;
                border: none;
                border-radius: 14px;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#summaryDataPanel {
                background-color: #173C47;
                border: none;
                border-radius: 14px;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#summaryAdvancedSection {
                background-color: #245D67;
                border: 2px solid #8FE8D1;
                border-radius: 8px;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#profileDivider {
                background: transparent;
                min-width: 2px;
                max-width: 2px;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#detailsPanel {
                background-color: #245D67;
                border: 2px solid #8FE8D1;
                border-radius: 8px;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#pokemonName {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 32pt;
                font-weight: bold;
                padding: 2px 6px 6px;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#pokemonInfo {
                background-color: #102A34;
                border: none;
                border-radius: 5px;
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 14pt;
                font-weight: bold;
                padding: 8px 10px;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#identityLabel {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 12pt;
                font-weight: bold;
                padding: 4px 6px;
                qproperty-alignment: AlignLeft;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#ivTitle,
            QFrame#pokemonCard[showcase="true"] QLabel#slotLabel {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 12pt;
                font-weight: bold;
                letter-spacing: 1.5px;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#shinyLabel {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 12pt;
                font-weight: bold;
            }

            QFrame#pokemonCard[showcase="true"] QProgressBar {
                min-height: 20px;
                max-height: 20px;
                border-radius: 9px;
                background-color: #102A34;
                border: 2px solid #8FE8D1;
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 11pt;
                font-weight: bold;
            }

            QFrame#pokemonCard[showcase="true"] QProgressBar::chunk {
                background-color: #2EB77C;
                border-radius: 0px;
            }

            QFrame#pokemonCard[showcase="true"] QFrame#separator {
                background-color: #8FE8D1;
                min-height: 2px;
                max-height: 2px;
            }

            QFrame#pokemonCard[showcase="true"] QLabel#statLabel {
                color: #FFFFFF;
                font-family: "Segoe UI", "Arial";
                font-size: 9pt;
                font-weight: bold;
                qproperty-alignment: AlignLeft;
            }

            QLabel#slotLabel {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
                letter-spacing: 1px;
            }

            QLabel#pokemonName {
                color: #FFFFFF;
                font-size: 21pt;
                font-weight: bold;
            }

            QLabel#pokemonInfo {
                color: #FFFFFF;
                font-size: 9pt;
            }

            QLabel#identityLabel {
                color: #FFFFFF;
                font-size: 8pt;
            }

            QLabel#shinyLabel {
                color: #FFFFFF;
                font-size: 9pt;
                font-weight: bold;
            }

            /* =========================================
               IV STATS
            ========================================= */

            QLabel#ivTitle {
                color: #FFFFFF;
                font-size: 10pt;
                font-weight: bold;
            }

            QLabel#statLabel {
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QProgressBar {
                background-color: rgba(0, 0, 0, 110);
                border: 1px solid rgba(255, 255, 255, 90);
                border-radius: 7px;
                min-height: 17px;
                max-height: 17px;
                text-align: center;
                color: #FFFFFF;
                font-size: 8pt;
                font-weight: bold;
            }

            QProgressBar::chunk {
                background-color: qlineargradient(
                    x1:0, y1:0, x2:0, y2:1,
                    stop:0 #90F890, stop:0.5 #48C848,
                    stop:1 #30A030
                );
                border-radius: 6px;
            }

            /* =========================================
               SEPARATORS
            ========================================= */

            QFrame#separator {
                background-color: rgba(255, 255, 255, 60);
                max-height: 1px;
                border: none;
            }
        """

        self.setStyleSheet(
            stylesheet.replace("__FONT__", self.game_font_family)
        )

    # ==========================================
    # CLOSE
    # ==========================================

    def closeEvent(self, event):
        """Play a short shutdown cue, then close without waiting for it."""

        if getattr(self, "closing", False):
            event.accept()
            return

        event.ignore()
        self.closing = True

        self.pc_player.stop()
        self.button_player.stop()

        if self.keyboard_timer.isActive():
            self.keyboard_timer.stop()

        if self.scanning:
            self.scanner.stop()
            self.scanning = False

        self.floating_pokemon_layer.stop()

        sound_path = self.sound_effects_dir / "pc-off.mp3"
        if sound_path.exists():
            self.pc_player.setSource(
                QUrl.fromLocalFile(str(sound_path))
            )
            self.pc_player.play()
            print("[Audio] Playing: pc-off.mp3")

        QTimer.singleShot(1000, self.finish_closing)

    def finish_closing(self):
        """Complete shutdown after the short audio lead-in."""

        self.close()