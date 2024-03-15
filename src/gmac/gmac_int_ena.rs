#[doc = "Register `GMAC_INT_ENA` reader"]
pub type R = crate::R<GmacIntEnaSpec>;
#[doc = "Register `GMAC_INT_ENA` writer"]
pub type W = crate::W<GmacIntEnaSpec>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
pub type TueR = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
pub type TueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
pub type TjeR = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
pub type TjeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
pub type UneR = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
pub type UneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type RueR = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
pub type RweR = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
pub type RweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (BIT 15), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
pub type EteR = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (BIT 15), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
pub type EteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
pub type EreR = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
pub type EreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    pub fn tue(&self) -> TueR {
        TueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn tje(&self) -> TjeR {
        TjeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
    #[inline(always)]
    pub fn une(&self) -> UneR {
        UneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn rue(&self) -> RueR {
        RueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn rwe(&self) -> RweR {
        RweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (BIT 15), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn ete(&self) -> EteR {
        EteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn ere(&self) -> EreR {
        EreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Interrupt is enabled. When this bit is reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<GmacIntEnaSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmission Stopped Interrupt is enabled. When this bit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<GmacIntEnaSpec> {
        TseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TueW<GmacIntEnaSpec> {
        TueW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TjeW<GmacIntEnaSpec> {
        TjeW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Overflow Interrupt is enabled. When this bit is reset, Overflow Interrupt is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OveW<GmacIntEnaSpec> {
        OveW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Transmit Underflow Interrupt is enabled. When this bit is reset, Underflow Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UneW<GmacIntEnaSpec> {
        UneW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Receive Interrupt is enabled. When this bit is reset, Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<GmacIntEnaSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RueW<GmacIntEnaSpec> {
        RueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), Receive Stopped Interrupt is enabled. When this bit is reset, Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<GmacIntEnaSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RweW<GmacIntEnaSpec> {
        RweW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (BIT 15), Early Transmit Interrupt is enabled. When this bit is reset, Early Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ete(&mut self) -> EteW<GmacIntEnaSpec> {
        EteW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FbeW<GmacIntEnaSpec> {
        FbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (BIT 16), Early Receive Interrupt is enabled. When this bit is reset, Early Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> EreW<GmacIntEnaSpec> {
        EreW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, an Abnormal Interrupt is enabled. When this bit is reset, an Abnormal Interrupt is disabled. This bit enables the following bits Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AieW<GmacIntEnaSpec> {
        AieW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, a normal interrupt is enabled. When this bit is reset, a normal interrupt is disabled. This bit enables the following bits: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NieW<GmacIntEnaSpec> {
        NieW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacIntEnaSpec;
impl crate::RegisterSpec for GmacIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_int_ena::R`](R) reader structure"]
impl crate::Readable for GmacIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_int_ena::W`](W) writer structure"]
impl crate::Writable for GmacIntEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_INT_ENA to value 0"]
impl crate::Resettable for GmacIntEnaSpec {
    const RESET_VALUE: u32 = 0;
}
