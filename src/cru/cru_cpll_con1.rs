#[doc = "Register `CRU_CPLL_CON1` reader"]
pub type R = crate::R<CruCpllCon1Spec>;
#[doc = "Register `CRU_CPLL_CON1` writer"]
pub type W = crate::W<CruCpllCon1Spec>;
#[doc = "Field `REFDIV` reader - Reference Clock Divide Value\n\n(1-63)"]
pub type RefdivR = crate::FieldReader;
#[doc = "Field `REFDIV` writer - Reference Clock Divide Value\n\n(1-63)"]
pub type RefdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `POSTDIV1` reader - First Post Divide Value\n\n(1-7)"]
pub type Postdiv1R = crate::FieldReader;
#[doc = "Field `POSTDIV1` writer - First Post Divide Value\n\n(1-7)"]
pub type Postdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POSTDIV2` reader - Second Post Divide Value\n\n(1-7)"]
pub type Postdiv2R = crate::FieldReader;
#[doc = "Field `POSTDIV2` writer - Second Post Divide Value\n\n(1-7)"]
pub type Postdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - Reference Clock Divide Value\n\n(1-63)"]
    #[inline(always)]
    pub fn refdiv(&self) -> RefdivR {
        RefdivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - First Post Divide Value\n\n(1-7)"]
    #[inline(always)]
    pub fn postdiv1(&self) -> Postdiv1R {
        Postdiv1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Second Post Divide Value\n\n(1-7)"]
    #[inline(always)]
    pub fn postdiv2(&self) -> Postdiv2R {
        Postdiv2R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Reference Clock Divide Value\n\n(1-63)"]
    #[inline(always)]
    #[must_use]
    pub fn refdiv(&mut self) -> RefdivW<CruCpllCon1Spec> {
        RefdivW::new(self, 0)
    }
    #[doc = "Bits 8:10 - First Post Divide Value\n\n(1-7)"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv1(&mut self) -> Postdiv1W<CruCpllCon1Spec> {
        Postdiv1W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Second Post Divide Value\n\n(1-7)"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv2(&mut self) -> Postdiv2W<CruCpllCon1Spec> {
        Postdiv2W::new(self, 12)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruCpllCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "CPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruCpllCon1Spec;
impl crate::RegisterSpec for CruCpllCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_cpll_con1::R`](R) reader structure"]
impl crate::Readable for CruCpllCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_cpll_con1::W`](W) writer structure"]
impl crate::Writable for CruCpllCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CPLL_CON1 to value 0x1302"]
impl crate::Resettable for CruCpllCon1Spec {
    const RESET_VALUE: u32 = 0x1302;
}
