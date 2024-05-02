#[doc = "Register `CPROC_HUE` reader"]
pub type R = crate::R<CprocHueSpec>;
#[doc = "Register `CPROC_HUE` writer"]
pub type W = crate::W<CprocHueSpec>;
#[doc = "Field `cproc_hue` reader - hue adjustment value 80H equals -90 deg\n\n…\n\n00H equals 0 deg\n\n…\n\n7FH equals +87.188 deg\n\n"]
pub type CprocHueR = crate::FieldReader;
#[doc = "Field `cproc_hue` writer - hue adjustment value 80H equals -90 deg\n\n…\n\n00H equals 0 deg\n\n…\n\n7FH equals +87.188 deg\n\n"]
pub type CprocHueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - hue adjustment value 80H equals -90 deg\n\n…\n\n00H equals 0 deg\n\n…\n\n7FH equals +87.188 deg\n\n"]
    #[inline(always)]
    pub fn cproc_hue(&self) -> CprocHueR {
        CprocHueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - hue adjustment value 80H equals -90 deg\n\n…\n\n00H equals 0 deg\n\n…\n\n7FH equals +87.188 deg\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_hue(&mut self) -> CprocHueW<CprocHueSpec> {
        CprocHueW::new(self, 0)
    }
}
#[doc = "Color Processing hue register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_hue::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_hue::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprocHueSpec;
impl crate::RegisterSpec for CprocHueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cproc_hue::R`](R) reader structure"]
impl crate::Readable for CprocHueSpec {}
#[doc = "`write(|w| ..)` method takes [`cproc_hue::W`](W) writer structure"]
impl crate::Writable for CprocHueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPROC_HUE to value 0"]
impl crate::Resettable for CprocHueSpec {
    const RESET_VALUE: u32 = 0;
}
