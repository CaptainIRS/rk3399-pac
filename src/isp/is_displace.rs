#[doc = "Register `IS_DISPLACE` reader"]
pub type R = crate::R<IsDisplaceSpec>;
#[doc = "Register `IS_DISPLACE` writer"]
pub type W = crate::W<IsDisplaceSpec>;
#[doc = "Field `dx` reader - ISP_IS will compensate for horizontal camera\n\ndisplacement of DX pixels in the next frame\n\n"]
pub type DxR = crate::FieldReader<u16>;
#[doc = "Field `dx` writer - ISP_IS will compensate for horizontal camera\n\ndisplacement of DX pixels in the next frame\n\n"]
pub type DxW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `dy` reader - ISP_IS will compensate for vertical camera\n\ndisplacement of DY lines in the next frame"]
pub type DyR = crate::FieldReader<u16>;
#[doc = "Field `dy` writer - ISP_IS will compensate for vertical camera\n\ndisplacement of DY lines in the next frame"]
pub type DyW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - ISP_IS will compensate for horizontal camera\n\ndisplacement of DX pixels in the next frame\n\n"]
    #[inline(always)]
    pub fn dx(&self) -> DxR {
        DxR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - ISP_IS will compensate for vertical camera\n\ndisplacement of DY lines in the next frame"]
    #[inline(always)]
    pub fn dy(&self) -> DyR {
        DyR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - ISP_IS will compensate for horizontal camera\n\ndisplacement of DX pixels in the next frame\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dx(&mut self) -> DxW<IsDisplaceSpec> {
        DxW::new(self, 0)
    }
    #[doc = "Bits 16:28 - ISP_IS will compensate for vertical camera\n\ndisplacement of DY lines in the next frame"]
    #[inline(always)]
    #[must_use]
    pub fn dy(&mut self) -> DyW<IsDisplaceSpec> {
        DyW::new(self, 16)
    }
}
#[doc = "Camera displacement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_displace::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_displace::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsDisplaceSpec;
impl crate::RegisterSpec for IsDisplaceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_displace::R`](R) reader structure"]
impl crate::Readable for IsDisplaceSpec {}
#[doc = "`write(|w| ..)` method takes [`is_displace::W`](W) writer structure"]
impl crate::Writable for IsDisplaceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_DISPLACE to value 0"]
impl crate::Resettable for IsDisplaceSpec {
    const RESET_VALUE: u32 = 0;
}
