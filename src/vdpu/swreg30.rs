#[doc = "Register `SWREG30` reader"]
pub type R = crate::R<Swreg30Spec>;
#[doc = "Register `SWREG30` writer"]
pub type W = crate::W<Swreg30Spec>;
#[doc = "Field `SW_DEINTERL_THR` reader - the threshold value of deinterlace"]
pub type SwDeinterlThrR = crate::FieldReader<u16>;
#[doc = "Field `SW_DEINTERL_THR` writer - the threshold value of deinterlace"]
pub type SwDeinterlThrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `SW_DEINTERL_EDGE` reader - the edge detect value of deinterlacing\n\nEdge detect value used for deinterlacing"]
pub type SwDeinterlEdgeR = crate::FieldReader<u16>;
#[doc = "Field `SW_DEINTERL_EDGE` writer - the edge detect value of deinterlacing\n\nEdge detect value used for deinterlacing"]
pub type SwDeinterlEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:13 - the threshold value of deinterlace"]
    #[inline(always)]
    pub fn sw_deinterl_thr(&self) -> SwDeinterlThrR {
        SwDeinterlThrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - the edge detect value of deinterlacing\n\nEdge detect value used for deinterlacing"]
    #[inline(always)]
    pub fn sw_deinterl_edge(&self) -> SwDeinterlEdgeR {
        SwDeinterlEdgeR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - the threshold value of deinterlace"]
    #[inline(always)]
    #[must_use]
    pub fn sw_deinterl_thr(&mut self) -> SwDeinterlThrW<Swreg30Spec> {
        SwDeinterlThrW::new(self, 0)
    }
    #[doc = "Bits 16:30 - the edge detect value of deinterlacing\n\nEdge detect value used for deinterlacing"]
    #[inline(always)]
    #[must_use]
    pub fn sw_deinterl_edge(&mut self) -> SwDeinterlEdgeW<Swreg30Spec> {
        SwDeinterlEdgeW::new(self, 16)
    }
}
#[doc = "register for deinterlace ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg30Spec;
impl crate::RegisterSpec for Swreg30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg30::R`](R) reader structure"]
impl crate::Readable for Swreg30Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg30::W`](W) writer structure"]
impl crate::Writable for Swreg30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG30 to value 0"]
impl crate::Resettable for Swreg30Spec {
    const RESET_VALUE: u32 = 0;
}
