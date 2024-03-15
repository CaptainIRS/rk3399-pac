#[doc = "Register `UART_LCR` reader"]
pub type R = crate::R<UartLcrSpec>;
#[doc = "Register `UART_LCR` writer"]
pub type W = crate::W<UartLcrSpec>;
#[doc = "Field `DATA_LENGTH_SEL` reader - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub type DataLengthSelR = crate::FieldReader;
#[doc = "Field `DATA_LENGTH_SEL` writer - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub type DataLengthSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_BITS_NUM` reader - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit."]
pub type StopBitsNumR = crate::BitReader;
#[doc = "Field `STOP_BITS_NUM` writer - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit."]
pub type StopBitsNumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN_PARITY_SEL` reader - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub type EvenParitySelR = crate::BitReader;
#[doc = "Field `EVEN_PARITY_SEL` writer - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub type EvenParitySelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_CTRL` reader - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If MCR\\[6\\]
set to one, the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub type BreakCtrlR = crate::BitReader;
#[doc = "Field `BREAK_CTRL` writer - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If MCR\\[6\\]
set to one, the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub type BreakCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_LAT_ACCESS` reader - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub type DivLatAccessR = crate::BitReader;
#[doc = "Field `DIV_LAT_ACCESS` writer - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub type DivLatAccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    pub fn data_length_sel(&self) -> DataLengthSelR {
        DataLengthSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit."]
    #[inline(always)]
    pub fn stop_bits_num(&self) -> StopBitsNumR {
        StopBitsNumR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn even_parity_sel(&self) -> EvenParitySelR {
        EvenParitySelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If MCR\\[6\\]
set to one, the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn break_ctrl(&self) -> BreakCtrlR {
        BreakCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn div_lat_access(&self) -> DivLatAccessR {
        DivLatAccessR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn data_length_sel(&mut self) -> DataLengthSelW<UartLcrSpec> {
        DataLengthSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit."]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits_num(&mut self) -> StopBitsNumW<UartLcrSpec> {
        StopBitsNumW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> ParityEnW<UartLcrSpec> {
        ParityEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    #[must_use]
    pub fn even_parity_sel(&mut self) -> EvenParitySelW<UartLcrSpec> {
        EvenParitySelW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If MCR\\[6\\]
set to one, the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    #[must_use]
    pub fn break_ctrl(&mut self) -> BreakCtrlW<UartLcrSpec> {
        BreakCtrlW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    #[must_use]
    pub fn div_lat_access(&mut self) -> DivLatAccessW<UartLcrSpec> {
        DivLatAccessW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartLcrSpec;
impl crate::RegisterSpec for UartLcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_lcr::R`](R) reader structure"]
impl crate::Readable for UartLcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_lcr::W`](W) writer structure"]
impl crate::Writable for UartLcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_LCR to value 0"]
impl crate::Resettable for UartLcrSpec {
    const RESET_VALUE: u32 = 0;
}
