
from PySide6.QtWidgets import QWidget, QVBoxLayout, QLabel

from .pokemon_card import PokemonCard


class PartyView(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(12)

        self.card = PokemonCard()

        self.status_label = QLabel(
            "Waiting for Party Slot 1..."
        )
        self.status_label.setObjectName("panelStatus")

        layout.addWidget(self.card, 1)
        layout.addWidget(self.status_label)

    def update_party(self, pokemon_list):
        lead_pokemon = next(
            (
                pokemon for pokemon in pokemon_list
                if pokemon.get("party_slot") == 1
            ),
            None
        )

        if lead_pokemon is None:
            self.card.set_empty()
            self.status_label.setText(
                "No Pokémon detected in Party Slot 1"
            )
            return

        self.card.set_pokemon(lead_pokemon)

        self.status_label.setText(
            "Party Slot 1 successfully detected"
        )
