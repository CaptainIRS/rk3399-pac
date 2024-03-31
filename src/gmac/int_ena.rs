#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Interrupt is enabled. When this bit is reset,\n\nTransmit Interrupt is disabled."]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Interrupt is enabled. When this bit is reset,\n\nTransmit Interrupt is disabled."]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmission Stopped Interrupt is enabled. When this\n\nbit is reset, Transmission Stopped Interrupt is disabled."]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmission Stopped Interrupt is enabled. When this\n\nbit is reset, Transmission Stopped Interrupt is disabled."]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unavailable Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Buffer Unavailable Interrupt is enabled. When this\n\nbit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
pub type TueR = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unavailable Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Buffer Unavailable Interrupt is enabled. When this\n\nbit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
pub type TueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Jabber Timeout Interrupt is enabled. When\n\nthis bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
pub type TjeR = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Jabber Timeout Interrupt is enabled. When\n\nthis bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
pub type TjeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Overflow Interrupt is enabled. When this bit is\n\nreset, Overflow Interrupt is disabled"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Overflow Interrupt is enabled. When this bit is\n\nreset, Overflow Interrupt is disabled"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Underflow Interrupt is enabled. When this bit\n\nis reset, Underflow Interrupt is disabled."]
pub type UneR = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Underflow Interrupt is enabled. When this bit\n\nis reset, Underflow Interrupt is disabled."]
pub type UneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Receive Interrupt is enabled. When this bit is reset, Receive\n\nInterrupt is disabled."]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Receive Interrupt is enabled. When this bit is reset, Receive\n\nInterrupt is disabled."]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Buffer Unavailable Interrupt is enabled. When\n\nthis bit is reset, the Receive Buffer Unavailable Interrupt is\n\ndisabled"]
pub type RueR = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Buffer Unavailable Interrupt is enabled. When\n\nthis bit is reset, the Receive Buffer Unavailable Interrupt is\n\ndisabled"]
pub type RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Stopped Interrupt is enabled. When this bit is\n\nreset, Receive Stopped Interrupt is disabled."]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Stopped Interrupt is enabled. When this bit is\n\nreset, Receive Stopped Interrupt is disabled."]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Receive Watchdog Timeout Interrupt is enabled.\n\nWhen this bit is reset, Receive\n\nWatchdog Timeout Interrupt is disabled."]
pub type RweR = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Receive Watchdog Timeout Interrupt is enabled.\n\nWhen this bit is reset, Receive\n\nWatchdog Timeout Interrupt is disabled."]
pub type RweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable\n\nWhen this bit is set with an Abnormal Interrupt Summary Enable\n\n(BIT 15), Early Transmit Interrupt is enabled. When this bit is\n\nreset, Early Transmit Interrupt is disabled."]
pub type EteR = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable\n\nWhen this bit is set with an Abnormal Interrupt Summary Enable\n\n(BIT 15), Early Transmit Interrupt is enabled. When this bit is\n\nreset, Early Transmit Interrupt is disabled."]
pub type EteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is\n\nreset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is\n\nreset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Early Receive Interrupt is enabled. When this bit is reset,\n\nEarly Receive Interrupt is disabled."]
pub type EreR = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Early Receive Interrupt is enabled. When this bit is reset,\n\nEarly Receive Interrupt is disabled."]
pub type EreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable\n\nWhen this bit is set, an Abnormal Interrupt is enabled. When this\n\nbit is reset, an Abnormal Interrupt is disabled. This bit enables\n\nthe following bits\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable\n\nWhen this bit is set, an Abnormal Interrupt is enabled. When this\n\nbit is reset, an Abnormal Interrupt is disabled. This bit enables\n\nthe following bits\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable\n\nWhen this bit is set, a normal interrupt is enabled. When this bit\n\nis reset, a normal interrupt is disabled. This bit enables the\n\nfollowing bits:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable\n\nWhen this bit is set, a normal interrupt is enabled. When this bit\n\nis reset, a normal interrupt is disabled. This bit enables the\n\nfollowing bits:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Interrupt is enabled. When this bit is reset,\n\nTransmit Interrupt is disabled."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmission Stopped Interrupt is enabled. When this\n\nbit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Buffer Unavailable Interrupt is enabled. When this\n\nbit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    pub fn tue(&self) -> TueR {
        TueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Jabber Timeout Interrupt is enabled. When\n\nthis bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn tje(&self) -> TjeR {
        TjeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Overflow Interrupt is enabled. When this bit is\n\nreset, Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Underflow Interrupt is enabled. When this bit\n\nis reset, Underflow Interrupt is disabled."]
    #[inline(always)]
    pub fn une(&self) -> UneR {
        UneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Receive Interrupt is enabled. When this bit is reset, Receive\n\nInterrupt is disabled."]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Buffer Unavailable Interrupt is enabled. When\n\nthis bit is reset, the Receive Buffer Unavailable Interrupt is\n\ndisabled"]
    #[inline(always)]
    pub fn rue(&self) -> RueR {
        RueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Stopped Interrupt is enabled. When this bit is\n\nreset, Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Receive Watchdog Timeout Interrupt is enabled.\n\nWhen this bit is reset, Receive\n\nWatchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn rwe(&self) -> RweR {
        RweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable\n\nWhen this bit is set with an Abnormal Interrupt Summary Enable\n\n(BIT 15), Early Transmit Interrupt is enabled. When this bit is\n\nreset, Early Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn ete(&self) -> EteR {
        EteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is\n\nreset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Early Receive Interrupt is enabled. When this bit is reset,\n\nEarly Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn ere(&self) -> EreR {
        EreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable\n\nWhen this bit is set, an Abnormal Interrupt is enabled. When this\n\nbit is reset, an Abnormal Interrupt is disabled. This bit enables\n\nthe following bits\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable\n\nWhen this bit is set, a normal interrupt is enabled. When this bit\n\nis reset, a normal interrupt is disabled. This bit enables the\n\nfollowing bits:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Interrupt is enabled. When this bit is reset,\n\nTransmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<IntEnaSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmission Stopped Interrupt is enabled. When this\n\nbit is reset, Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<IntEnaSpec> {
        TseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Transmit Buffer Unavailable Interrupt is enabled. When this\n\nbit is reset, Transmit Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TueW<IntEnaSpec> {
        TueW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Jabber Timeout Interrupt is enabled. When\n\nthis bit is reset, Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TjeW<IntEnaSpec> {
        TjeW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Overflow Interrupt is enabled. When this bit is\n\nreset, Overflow Interrupt is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OveW<IntEnaSpec> {
        OveW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Transmit Underflow Interrupt is enabled. When this bit\n\nis reset, Underflow Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UneW<IntEnaSpec> {
        UneW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Receive Interrupt is enabled. When this bit is reset, Receive\n\nInterrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<IntEnaSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Buffer Unavailable Interrupt is enabled. When\n\nthis bit is reset, the Receive Buffer Unavailable Interrupt is\n\ndisabled"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RueW<IntEnaSpec> {
        RueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), Receive Stopped Interrupt is enabled. When this bit is\n\nreset, Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<IntEnaSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Receive Watchdog Timeout Interrupt is enabled.\n\nWhen this bit is reset, Receive\n\nWatchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RweW<IntEnaSpec> {
        RweW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable\n\nWhen this bit is set with an Abnormal Interrupt Summary Enable\n\n(BIT 15), Early Transmit Interrupt is enabled. When this bit is\n\nreset, Early Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ete(&mut self) -> EteW<IntEnaSpec> {
        EteW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable\n\nWhen this bit is set with Abnormal Interrupt Summary Enable\n\n(BIT 15), the Fatal Bus Error Interrupt is enabled. When this bit is\n\nreset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FbeW<IntEnaSpec> {
        FbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable\n\nWhen this bit is set with Normal Interrupt Summary Enable (BIT\n\n16), Early Receive Interrupt is enabled. When this bit is reset,\n\nEarly Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> EreW<IntEnaSpec> {
        EreW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable\n\nWhen this bit is set, an Abnormal Interrupt is enabled. When this\n\nbit is reset, an Abnormal Interrupt is disabled. This bit enables\n\nthe following bits\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AieW<IntEnaSpec> {
        AieW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable\n\nWhen this bit is set, a normal interrupt is enabled. When this bit\n\nis reset, a normal interrupt is disabled. This bit enables the\n\nfollowing bits:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NieW<IntEnaSpec> {
        NieW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {
    const RESET_VALUE: u32 = 0;
}
