#[doc = "Register `CRU_VPLL_CON0` reader"]
pub type R = crate::R<CruVpllCon0Spec>;
#[doc = "Register `CRU_VPLL_CON0` writer"]
pub type W = crate::W<CruVpllCon0Spec>;
#[doc = "Field `FBDIV` reader - Feedback Divide Value\n\nValid divider settings are:\n\n\\[16, 3200\\]
in integer mode\n\n\\[20, 320\\]
in fractional mode\n\nTips: no plus one operation"]
pub type FbdivR = crate::FieldReader<u16>;
#[doc = "Field `FBDIV` writer - Feedback Divide Value\n\nValid divider settings are:\n\n\\[16, 3200\\]
in integer mode\n\n\\[20, 320\\]
in fractional mode\n\nTips: no plus one operation"]
pub type FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - Feedback Divide Value\n\nValid divider settings are:\n\n\\[16, 3200\\]
in integer mode\n\n\\[20, 320\\]
in fractional mode\n\nTips: no plus one operation"]
    #[inline(always)]
    pub fn fbdiv(&self) -> FbdivR {
        FbdivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Feedback Divide Value\n\nValid divider settings are:\n\n\\[16, 3200\\]
in integer mode\n\n\\[20, 320\\]
in fractional mode\n\nTips: no plus one operation"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv(&mut self) -> FbdivW<CruVpllCon0Spec> {
        FbdivW::new(self, 0)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruVpllCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "VPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruVpllCon0Spec;
impl crate::RegisterSpec for CruVpllCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_vpll_con0::R`](R) reader structure"]
impl crate::Readable for CruVpllCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_vpll_con0::W`](W) writer structure"]
impl crate::Writable for CruVpllCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_VPLL_CON0 to value 0xc6"]
impl crate::Resettable for CruVpllCon0Spec {
    const RESET_VALUE: u32 = 0xc6;
}
