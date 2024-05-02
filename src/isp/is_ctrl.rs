#[doc = "Register `IS_CTRL` reader"]
pub type R = crate::R<IsCtrlSpec>;
#[doc = "Register `IS_CTRL` writer"]
pub type W = crate::W<IsCtrlSpec>;
#[doc = "Field `is_en` reader - 1: image stabilization\n\nswitched on 0: image\n\nstabilization switched off\n\n"]
pub type IsEnR = crate::BitReader;
#[doc = "Field `is_en` writer - 1: image stabilization\n\nswitched on 0: image\n\nstabilization switched off\n\n"]
pub type IsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: image stabilization\n\nswitched on 0: image\n\nstabilization switched off\n\n"]
    #[inline(always)]
    pub fn is_en(&self) -> IsEnR {
        IsEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: image stabilization\n\nswitched on 0: image\n\nstabilization switched off\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_en(&mut self) -> IsEnW<IsCtrlSpec> {
        IsEnW::new(self, 0)
    }
}
#[doc = "Image Stabilization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsCtrlSpec;
impl crate::RegisterSpec for IsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_ctrl::R`](R) reader structure"]
impl crate::Readable for IsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`is_ctrl::W`](W) writer structure"]
impl crate::Writable for IsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_CTRL to value 0"]
impl crate::Resettable for IsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
