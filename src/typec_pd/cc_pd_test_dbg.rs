#[doc = "Register `CC_PD_TEST_DBG` reader"]
pub type R = crate::R<CcPdTestDbgSpec>;
#[doc = "Register `CC_PD_TEST_DBG` writer"]
pub type W = crate::W<CcPdTestDbgSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSerialLoopbackEnable {
    #[doc = "1: Enable PD Tx to Rx serial loopback."]
    B1 = 1,
    #[doc = "0: Tx to Rx loopback disabled."]
    B0 = 0,
}
impl From<PdSerialLoopbackEnable> for bool {
    #[inline(always)]
    fn from(variant: PdSerialLoopbackEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_Serial_Loopback_enable` reader - "]
pub type PdSerialLoopbackEnableR = crate::BitReader<PdSerialLoopbackEnable>;
impl PdSerialLoopbackEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSerialLoopbackEnable {
        match self.bits {
            true => PdSerialLoopbackEnable::B1,
            false => PdSerialLoopbackEnable::B0,
        }
    }
    #[doc = "Enable PD Tx to Rx serial loopback."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdSerialLoopbackEnable::B1
    }
    #[doc = "Tx to Rx loopback disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdSerialLoopbackEnable::B0
    }
}
#[doc = "Field `PD_Serial_Loopback_enable` writer - "]
pub type PdSerialLoopbackEnableW<'a, REG> = crate::BitWriter<'a, REG, PdSerialLoopbackEnable>;
impl<'a, REG> PdSerialLoopbackEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable PD Tx to Rx serial loopback."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdSerialLoopbackEnable::B1)
    }
    #[doc = "Tx to Rx loopback disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdSerialLoopbackEnable::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd_serial_loopback_enable(&self) -> PdSerialLoopbackEnableR {
        PdSerialLoopbackEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd_serial_loopback_enable(&mut self) -> PdSerialLoopbackEnableW<CcPdTestDbgSpec> {
        PdSerialLoopbackEnableW::new(self, 0)
    }
}
#[doc = "CC PD Test Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_pd_test_dbg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_pd_test_dbg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcPdTestDbgSpec;
impl crate::RegisterSpec for CcPdTestDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_pd_test_dbg::R`](R) reader structure"]
impl crate::Readable for CcPdTestDbgSpec {}
#[doc = "`write(|w| ..)` method takes [`cc_pd_test_dbg::W`](W) writer structure"]
impl crate::Writable for CcPdTestDbgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_PD_TEST_DBG to value 0"]
impl crate::Resettable for CcPdTestDbgSpec {
    const RESET_VALUE: u32 = 0;
}
