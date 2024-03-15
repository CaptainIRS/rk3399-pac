#[doc = "Register `UART_IER` reader"]
pub type R = crate::R<UartIerSpec>;
#[doc = "Register `UART_IER` writer"]
pub type W = crate::W<UartIerSpec>;
#[doc = "Field `RECEIVE_DATA_AVAILABLE_INT_EN` reader - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
pub type ReceiveDataAvailableIntEnR = crate::BitReader;
#[doc = "Field `RECEIVE_DATA_AVAILABLE_INT_EN` writer - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
pub type ReceiveDataAvailableIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_HOLD_EMPTY_INT_EN` reader - Enable Transmit Holding Register Empty Interrupt."]
pub type TransHoldEmptyIntEnR = crate::BitReader;
#[doc = "Field `TRANS_HOLD_EMPTY_INT_EN` writer - Enable Transmit Holding Register Empty Interrupt."]
pub type TransHoldEmptyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_LINE_STATUS_INT_EN` reader - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ReceiveLineStatusIntEnR = crate::BitReader;
#[doc = "Field `RECEIVE_LINE_STATUS_INT_EN` writer - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ReceiveLineStatusIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_STATUS_INT_EN` reader - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ModemStatusIntEnR = crate::BitReader;
#[doc = "Field `MODEM_STATUS_INT_EN` writer - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ModemStatusIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_THRE_INT_EN` reader - Programmable THRE Interrupt Mode Enable This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
pub type ProgThreIntEnR = crate::BitReader;
#[doc = "Field `PROG_THRE_INT_EN` writer - Programmable THRE Interrupt Mode Enable This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
pub type ProgThreIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn receive_data_available_int_en(&self) -> ReceiveDataAvailableIntEnR {
        ReceiveDataAvailableIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt."]
    #[inline(always)]
    pub fn trans_hold_empty_int_en(&self) -> TransHoldEmptyIntEnR {
        TransHoldEmptyIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn receive_line_status_int_en(&self) -> ReceiveLineStatusIntEnR {
        ReceiveLineStatusIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn modem_status_int_en(&self) -> ModemStatusIntEnR {
        ModemStatusIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn prog_thre_int_en(&self) -> ProgThreIntEnR {
        ProgThreIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn receive_data_available_int_en(&mut self) -> ReceiveDataAvailableIntEnW<UartIerSpec> {
        ReceiveDataAvailableIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_hold_empty_int_en(&mut self) -> TransHoldEmptyIntEnW<UartIerSpec> {
        TransHoldEmptyIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn receive_line_status_int_en(&mut self) -> ReceiveLineStatusIntEnW<UartIerSpec> {
        ReceiveLineStatusIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn modem_status_int_en(&mut self) -> ModemStatusIntEnW<UartIerSpec> {
        ModemStatusIntEnW::new(self, 3)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn prog_thre_int_en(&mut self) -> ProgThreIntEnW<UartIerSpec> {
        ProgThreIntEnW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIerSpec;
impl crate::RegisterSpec for UartIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ier::R`](R) reader structure"]
impl crate::Readable for UartIerSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_ier::W`](W) writer structure"]
impl crate::Writable for UartIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_IER to value 0"]
impl crate::Resettable for UartIerSpec {
    const RESET_VALUE: u32 = 0;
}
