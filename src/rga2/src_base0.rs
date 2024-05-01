#[doc = "Register `SRC_BASE0` reader"]
pub type R = crate::R<SrcBase0Spec>;
#[doc = "Register `SRC_BASE0` writer"]
pub type W = crate::W<SrcBase0Spec>;
#[doc = "Field `SW_SRC_BASE0` reader - source image Y/RGB base address"]
pub type SwSrcBase0R = crate::FieldReader<u32>;
#[doc = "Field `SW_SRC_BASE0` writer - source image Y/RGB base address"]
pub type SwSrcBase0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source image Y/RGB base address"]
    #[inline(always)]
    pub fn sw_src_base0(&self) -> SwSrcBase0R {
        SwSrcBase0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source image Y/RGB base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_base0(&mut self) -> SwSrcBase0W<SrcBase0Spec> {
        SwSrcBase0W::new(self, 0)
    }
}
#[doc = "source image Y/RGB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBase0Spec;
impl crate::RegisterSpec for SrcBase0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_base0::R`](R) reader structure"]
impl crate::Readable for SrcBase0Spec {}
#[doc = "`write(|w| ..)` method takes [`src_base0::W`](W) writer structure"]
impl crate::Writable for SrcBase0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_BASE0 to value 0"]
impl crate::Resettable for SrcBase0Spec {
    const RESET_VALUE: u32 = 0;
}
