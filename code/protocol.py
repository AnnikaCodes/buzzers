### Protocol Messages
# Red buzzes
RED_TEAM_BUZZES = ['F', 'G', 'H', 'I', 'J']
# Green buzzes
GREEN_TEAM_BUZZES = ['A', 'B', 'C', 'D', 'E']
# Lockouts
RED_TEAM_LOCKOUTS = [x.lower() for x in RED_TEAM_BUZZES]
GREEN_TEAM_LOCKOUTS = [x.lower() for x in GREEN_TEAM_BUZZES]
CLEAR = 'x'