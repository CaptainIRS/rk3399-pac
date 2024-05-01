#[doc = "Register `WIN0_VIR` reader"]
pub type R = crate::R<Win0VirSpec>;
#[doc = "Register `WIN0_VIR` writer"]
pub type W = crate::W<Win0VirSpec>;
#[doc = "Field `WIN0_VIR_STRIDE` reader - Win0 Virtual stride\n\nNumber of words of Win0 yrgb Virtual width\n\nARGB888 : win0_vir_width\n\nRGB888 : (win0_vir_width*3/4) + (win0_vir_width%3)\n\nRGB565 : ceil(win0_vir_width/2)\n\nYUV : ceil(win0_vir_width/4)"]
pub type Win0VirStrideR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_VIR_STRIDE` writer - Win0 Virtual stride\n\nNumber of words of Win0 yrgb Virtual width\n\nARGB888 : win0_vir_width\n\nRGB888 : (win0_vir_width*3/4) + (win0_vir_width%3)\n\nRGB565 : ceil(win0_vir_width/2)\n\nYUV : ceil(win0_vir_width/4)"]
pub type Win0VirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN0_VIR_STRIDE_UV` reader - Number of words of Win0 uv Virtual width"]
pub type Win0VirStrideUvR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_VIR_STRIDE_UV` writer - Number of words of Win0 uv Virtual width"]
pub type Win0VirStrideUvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win0 Virtual stride\n\nNumber of words of Win0 yrgb Virtual width\n\nARGB888 : win0_vir_width\n\nRGB888 : (win0_vir_width*3/4) + (win0_vir_width%3)\n\nRGB565 : ceil(win0_vir_width/2)\n\nYUV : ceil(win0_vir_width/4)"]
    #[inline(always)]
    pub fn win0_vir_stride(&self) -> Win0VirStrideR {
        Win0VirStrideR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of words of Win0 uv Virtual width"]
    #[inline(always)]
    pub fn win0_vir_stride_uv(&self) -> Win0VirStrideUvR {
        Win0VirStrideUvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win0 Virtual stride\n\nNumber of words of Win0 yrgb Virtual width\n\nARGB888 : win0_vir_width\n\nRGB888 : (win0_vir_width*3/4) + (win0_vir_width%3)\n\nRGB565 : ceil(win0_vir_width/2)\n\nYUV : ceil(win0_vir_width/4)"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vir_stride(&mut self) -> Win0VirStrideW<Win0VirSpec> {
        Win0VirStrideW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of words of Win0 uv Virtual width"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vir_stride_uv(&mut self) -> Win0VirStrideUvW<Win0VirSpec> {
        Win0VirStrideUvW::new(self, 16)
    }
}
#[doc = "Win0 virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_vir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_vir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0VirSpec;
impl crate::RegisterSpec for Win0VirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_vir::R`](R) reader structure"]
impl crate::Readable for Win0VirSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_vir::W`](W) writer structure"]
impl crate::Writable for Win0VirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_VIR to value 0x0140_0140"]
impl crate::Resettable for Win0VirSpec {
    const RESET_VALUE: u32 = 0x0140_0140;
}
