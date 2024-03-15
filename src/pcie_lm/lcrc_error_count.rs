#[doc = "Register `LCRC_ERROR_COUNT` reader"]
pub type R = crate::R<LcrcErrorCountSpec>;
#[doc = "Register `LCRC_ERROR_COUNT` writer"]
pub type W = crate::W<LcrcErrorCountSpec>;
#[doc = "Field `LEC` reader - LCRC Eror Count \\[LEC\\]
Number of TLPs received with LCRC errors."]
pub type LecR = crate::FieldReader<u16>;
#[doc = "Field `LEC` writer - LCRC Eror Count \\[LEC\\]
Number of TLPs received with LCRC errors."]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R11` reader - Reserved \\[R11\\]
Reserved"]
pub type R11R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LCRC Eror Count \\[LEC\\]
Number of TLPs received with LCRC errors."]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R11\\]
Reserved"]
    #[inline(always)]
    pub fn r11(&self) -> R11R {
        R11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LCRC Eror Count \\[LEC\\]
Number of TLPs received with LCRC errors."]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<LcrcErrorCountSpec> {
        LecW::new(self, 0)
    }
}
#[doc = "LCRC Error Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrc_error_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrc_error_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrcErrorCountSpec;
impl crate::RegisterSpec for LcrcErrorCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrc_error_count::R`](R) reader structure"]
impl crate::Readable for LcrcErrorCountSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrc_error_count::W`](W) writer structure"]
impl crate::Writable for LcrcErrorCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets LCRC_ERROR_COUNT to value 0"]
impl crate::Resettable for LcrcErrorCountSpec {
    const RESET_VALUE: u32 = 0;
}
