# Llibraries + Utilitzem la taula que està a la mateixa carpeta que l'script
library(readxl)
library(ggplot2)
library(emmeans)
# Si dona error amb l'xlsx, fer getwd() per veure des de quin directori està agafant la info
# i si no és el correcte corregir-lo amb setwd(). Per exemple:
# getwd() = "C:/Users/alber_rpsdl4k/Documents"
# setwd("C:/Users/alber_rpsdl4k/Documents/Bogomult") -> Que es on està (en el meu cas) l'xlsx
X23A_Cuixart_Gil_Mur_DADES <- read_excel("23A_Cuixart_Gil_Mur.xlsx")

# Bogomult
mitjana_bogomult <- mean(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)`,na.rm=TRUE) 
mediana_bogomult <- median(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)`,na.rm=TRUE)
sd_bogomult <- sd(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)`,na.rm=TRUE)
cat("Per Bogomult, la mitjana es de ", mitjana_bogomult, ", la mediana de ", mediana_bogomult, " i l'error estàndard de ",
    sd_bogomult, ".\n")

# MUL Rust
mitjana_rust <- mean(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`,na.rm=TRUE)
mediana_rust <- median(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`,na.rm=TRUE)
sd_rust <- sd(X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`,na.rm=TRUE)
cat("Per MUL Rust, la mitjana es de ", mitjana_rust, ", la mediana es de ", mediana_rust, " i l'error estàndard de ",
    sd_rust, ".\n")

# Càlcul IC
diferencies <- X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)` - X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`
mitjana_diferencies <- mean(diferencies, na.rm = TRUE)
sd_diferencies <- sd(diferencies, na.rm = TRUE)
n <- length(diferencies[!is.na(diferencies)])
se_diferencies <- sd_diferencies / sqrt(n)
t_student <- qt(0.975, df = n - 1) # Nivell de confiança del 95%
limit_inferior <- mitjana_diferencies - t_student * se_diferencies
limit_superior <- mitjana_diferencies + t_student * se_diferencies
cat("IC de la diferència de temps (95%): [", limit_inferior, ",",limit_superior, "]\n")

# Això ens indica que 
# 1. Hi ha una diferència molt consistent entre els temps dels algorismes:  
#    amb un 95% de confiança, la diferència promig entre els temps dels dos algorismes
#    està entre 18.51633 i 18.52058, la qual cosa és un indicador de la robustesa dels resultats
# 2. L'algorisme de MUL Rust és significativament més ràpid que Bogomult :( , amb una diferència
#    promig de 18.52 nanosegons 

# Sponsored by CHATGPT
# Cargar librerías necesarias
library(ggplot2)

# Calcular las diferencias y los promedios
diferencias <- X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)` - X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`
promedios <- (X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)` + X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`) / 2

# Calcular la media y desviación estándar de las diferencias
media_diferencias <- mean(diferencias, na.rm = TRUE)
sd_diferencias <- sd(diferencias, na.rm = TRUE)

# Calcular los límites de acuerdo (±1.96 * SD)
limite_inferior <- media_diferencias - 1.96 * sd_diferencias
limite_superior <- media_diferencias + 1.96 * sd_diferencias

# Crear un dataframe para ggplot
df_bland_altman <- data.frame(Promedio = promedios, Diferencia = diferencias)

# Graficar con ggplot
dev.new()
dev.new()
ggplot(df_bland_altman, aes(x = Promedio, y = Diferencia)) +
  geom_point() +  # Puntos de la gráfica
  geom_hline(yintercept = media_diferencias, linetype = "dashed", color = "blue") +  # Línea de media
  geom_hline(yintercept = limite_inferior, linetype = "dashed", color = "red") +  # Línea límite inferior
  geom_hline(yintercept = limite_superior, linetype = "dashed", color = "red") +  # Línea límite superior
  xlab("Promedio de los métodos") +  # Etiqueta eje X
  ylab("Diferencia entre los métodos") +  # Etiqueta eje Y
  ggtitle("Gráfico de Bland-Altman") +  # Título del gráfico
  theme_minimal()  # Estilo minimalista

# Gràfic QQ de les diferències
# Propòsit: Evalua si el conjunt de les dades segueix una distribució normal
dev.new()
qqnorm(diferencias, main = "Gráfico QQ de las Diferencias", col = "blue", pch = 19)  # Puntos azules
qqline(diferencias, col = "red", lwd = 2)  # Línea roja que indica la normalidad

X <- X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Bogomult en nanosegons)`
Y <- X23A_Cuixart_Gil_Mur_DADES$`Log2(Temps_Mul en nanosegons)`

# Model lineal simple i múltiple
# Plot 1: Gràfic QQ
# Propòsit: Verifica si els residus del model es distribueixen de forma normal  
dev.new() 
plot(lm(Y ~ X), which = 2)  # Normal Q-Q plot
dev.new()  # Open a new graphics device for the next plot

plot(lm(Y ~ X), which = 1)  # Residuals vs Fitted plot
dev.new()  # Open another new graphics device

# Plot 2: Histogram of standardized residuals
hist(rstandard(lm(Y ~ X)), main = "Histogram of Standardized Residuals", xlab = "Standardized Residuals", col = "lightblue", border = "black")
dev.new()

# Plot 3: Standardized residuals over index
plot(rstandard(lm(Y ~ X)), type = "l", main = "Standardized Residuals", xlab = "Index", ylab = "Standardized Residuals", col = "blue")





