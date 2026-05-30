/**
  ******************************************************************************
  * @file stm8_interrupt_vector.c
  * @brief This file contains basic interrupt vector table for STM8 devices.
  * @author STMicroelectronics - MCD Application Team
  * @version V1.0.2
  * @date APR-2010
  ******************************************************************************
  *
  * THE PRESENT FIRMWARE WHICH IS FOR GUIDANCE ONLY AIMS AT PROVIDING CUSTOMERS
  * WITH CODING INFORMATION REGARDING THEIR PRODUCTS IN ORDER FOR THEM TO SAVE
  * TIME. AS A RESULT, STMICROELECTRONICS SHALL NOT BE HELD LIABLE FOR ANY
  * DIRECT, INDIRECT OR CONSEQUENTIAL DAMAGES WITH RESPECT TO ANY CLAIMS ARISING
  * FROM THE CONTENT OF SUCH FIRMWARE AND/OR THE USE MADE BY CUSTOMERS OF THE
  * CODING INFORMATION CONTAINED HEREIN IN CONNECTION WITH THEIR PRODUCTS.
  *
  * <h2><center>&copy; COPYRIGHT 2009 STMicroelectronics</center></h2>
  * @image html logo.bmp
  ******************************************************************************
  */

/* Includes ------------------------------------------------------------------*/
#include "stm8s_it.h"
#include "stm8_stl_param.h"
#include "stm8_stl_startup.h"

typedef void @far (*interrupt_handler_t)(void);

struct interrupt_vector {
	u8 interrupt_instruction;
	interrupt_handler_t interrupt_handler;
};

#ifdef STL_INCL_POR
extern void STL_StartUp(); /* Class B startup routine */
#else
extern void _stext();      /* standard startup routine */
#endif

struct interrupt_vector const _vectab[] = {
#ifdef STL_INCL_POR
  {0x82, (interrupt_handler_t)STL_StartUp}, /* RESET */
#else
  {0x82, (interrupt_handler_t)_stext},      /* RESET */
#endif
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* TRAP - Software interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq0 - External Top Level interrupt (TLI) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq1 - Auto Wake Up from Halt interrupt */
	{0x82, (interrupt_handler_t)CLK_IRQHandler},      /* irq2 - Clock Controller interrupt */
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq3 - External interrupt 0 (GPIOA) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq4 - External interrupt 1 (GPIOB) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq5 - External interrupt 2 (GPIOC) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq6 - External interrupt 3 (GPIOD) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq7 - External interrupt 4 (GPIOE) */
	
#ifdef STM8S208
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq8 - CAN Rx interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq9 - CAN Tx/ER/SC interrupt */
#elif defined (STM8S903)
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq8 - External interrupt 5 (GPIOF) */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq9 - Reserved */
#else /*STM8S207, STM8S105 or STM8S103*/
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq8 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq9 - Reserved */
#endif /*STM8S208*/
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq10 - SPI End of transfer interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq11 - TIM1 Update/Overflow/Trigger/Break interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq12 - TIM1 Capture/Compare interrupt */
  
#ifdef STM8S903
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq13 - TIM5 Update/Overflow/Break/Trigger interrupt  */
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq14 - TIM5 Capture/Compare interrupt */
	
#else /*STM8S208, STM8S207, STM8S105 or STM8S103*/
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq13 - TIM2 Update/Overflow/Break interrupt  */
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq14 - TIM2 Capture/Compare interrupt */
#endif /*STM8S903*/
	
#if defined (STM8S208) || defined(STM8S207) || defined(STM8S105)
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq15 - TIM3 Update/Overflow/Break interrupt */
  {0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq16 - TIM3 Capture/Compare interrupt */
#else
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq15 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq16 - Reserved */
#endif /*STM8S208, STM8S207 or STM8S105*/
	
#ifdef STM8S105
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq17 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq18 - Reserved */
#else
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq17 - UART1 Tx complete interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq18 - UART1 Rx interrupt */
#endif /*STM8S105*/
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq19 - I2C interrupt */

#if defined(STM8S208) || defined(STM8S207)

	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq20 - UART3 Tx interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq21 - UART3 Rx interrupt */
#elif defined (STM8S105)
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq20 - UART2 Tx interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq21 - UART2 Rx interrupt */

#else /* STM8S103, STM8S903 */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq20 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq21 - Reserved */
#endif /* STM8S208, STM8S207 */

#if defined(STM8S208) || defined(STM8S207)
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq22 - ADC2 end of conversion interrupt */
#else /* STM8S105, STM8S103, STM8S903 */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq22 - ADC1 end of conversion/Analog watchdog interrupts */

#endif /* STM8S208, STM8S207 */

#ifdef STM8S903
	{0x82, (interrupt_handler_t)TIM6_UPD_OVF_TRG_IRQHandler}, /* irq23 - TIM6 Update/Overflow/Trigger interrupt */
#else
	{0x82, (interrupt_handler_t)TIM4_UPD_OVF_IRQHandler}, /* irq23 - TIM4 Update/Overflow interrupt */
#endif /*STM8S903*/
	{0x82, (interrupt_handler_t)NonHandledInterrupt},  /* irq24 - FLASH interrupt */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq25 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq26 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq27 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq28 - Reserved */
	{0x82, (interrupt_handler_t)NonHandledInterrupt}, /* irq29 - Reserved */

};
/**
  * @}
  */
/******************* (C) COPYRIGHT 2009 STMicroelectronics *****END OF FILE****/