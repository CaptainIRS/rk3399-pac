#[doc = "Register `MINTSTS` reader"]
pub type R = crate::R<MintstsSpec>;
#[doc = "Register `MINTSTS` writer"]
pub type W = crate::W<MintstsSpec>;
#[doc = "Field `INT_STATUS` reader - Interrupt enabled only if corresponding bit in interrupt mask\n\nregister is set.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
pub type IntStatusR = crate::FieldReader<u16>;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` reader - Data no busy Interrupt Status"]
pub type DataNobusyIntStatusR = crate::BitReader;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` writer - Data no busy Interrupt Status"]
pub type DataNobusyIntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt from SDIO card; SDIO interrupt for card enabled only if\n\ncorresponding sdio_int_mask bit is set in Interrupt mask register\n\n(mask bit 1 enables interrupt; 0 masks interrupt).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioInterrupt {
    #[doc = "0: no SDIO interrupt from card"]
    B0 = 0,
    #[doc = "1: SDIO interrupt from card"]
    B1 = 1,
}
impl From<SdioInterrupt> for bool {
    #[inline(always)]
    fn from(variant: SdioInterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_INTERRUPT` reader - Interrupt from SDIO card; SDIO interrupt for card enabled only if\n\ncorresponding sdio_int_mask bit is set in Interrupt mask register\n\n(mask bit 1 enables interrupt; 0 masks interrupt)."]
pub type SdioInterruptR = crate::BitReader<SdioInterrupt>;
impl SdioInterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioInterrupt {
        match self.bits {
            false => SdioInterrupt::B0,
            true => SdioInterrupt::B1,
        }
    }
    #[doc = "no SDIO interrupt from card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdioInterrupt::B0
    }
    #[doc = "SDIO interrupt from card"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdioInterrupt::B1
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt enabled only if corresponding bit in interrupt mask\n\nregister is set.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Data no busy Interrupt Status"]
    #[inline(always)]
    pub fn data_nobusy_int_status(&self) -> DataNobusyIntStatusR {
        DataNobusyIntStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt from SDIO card; SDIO interrupt for card enabled only if\n\ncorresponding sdio_int_mask bit is set in Interrupt mask register\n\n(mask bit 1 enables interrupt; 0 masks interrupt)."]
    #[inline(always)]
    pub fn sdio_interrupt(&self) -> SdioInterruptR {
        SdioInterruptR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Data no busy Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn data_nobusy_int_status(&mut self) -> DataNobusyIntStatusW<MintstsSpec> {
        DataNobusyIntStatusW::new(self, 16)
    }
}
#[doc = "Masked interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintstsSpec;
impl crate::RegisterSpec for MintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintsts::R`](R) reader structure"]
impl crate::Readable for MintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`mintsts::W`](W) writer structure"]
impl crate::Writable for MintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MINTSTS to value 0"]
impl crate::Resettable for MintstsSpec {
    const RESET_VALUE: u32 = 0;
}
