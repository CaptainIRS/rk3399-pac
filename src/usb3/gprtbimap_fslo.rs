#[doc = "Register `GPRTBIMAP_FSLO` reader"]
pub type R = crate::R<GprtbimapFsloSpec>;
#[doc = "Register `GPRTBIMAP_FSLO` writer"]
pub type W = crate::W<GprtbimapFsloSpec>;
#[doc = "Field `BINUM1` reader - FS USB Instance Number for Port 1\n\nApplication-programmable ID field."]
pub type Binum1R = crate::FieldReader;
#[doc = "Field `BINUM1` writer - FS USB Instance Number for Port 1\n\nApplication-programmable ID field."]
pub type Binum1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - FS USB Instance Number for Port 1\n\nApplication-programmable ID field."]
    #[inline(always)]
    pub fn binum1(&self) -> Binum1R {
        Binum1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FS USB Instance Number for Port 1\n\nApplication-programmable ID field."]
    #[inline(always)]
    #[must_use]
    pub fn binum1(&mut self) -> Binum1W<GprtbimapFsloSpec> {
        Binum1W::new(self, 0)
    }
}
#[doc = "Global Full-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gprtbimap_fslo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gprtbimap_fslo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GprtbimapFsloSpec;
impl crate::RegisterSpec for GprtbimapFsloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gprtbimap_fslo::R`](R) reader structure"]
impl crate::Readable for GprtbimapFsloSpec {}
#[doc = "`write(|w| ..)` method takes [`gprtbimap_fslo::W`](W) writer structure"]
impl crate::Writable for GprtbimapFsloSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPRTBIMAP_FSLO to value 0"]
impl crate::Resettable for GprtbimapFsloSpec {
    const RESET_VALUE: u32 = 0;
}
