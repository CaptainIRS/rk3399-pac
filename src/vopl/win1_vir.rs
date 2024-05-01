#[doc = "Register `WIN1_VIR` reader"]
pub type R = crate::R<Win1VirSpec>;
#[doc = "Register `WIN1_VIR` writer"]
pub type W = crate::W<Win1VirSpec>;
#[doc = "Field `WIN1_VIR_STRIDE` reader - Win1 Virtual stride\n\nNumber of words of Win1 yrgb Virtual width\n\nARGB888 : win1_vir_width\n\nRGB888 : (win1_vir_width*3/4) + (win1_vir_width%3)\n\nRGB565 : ceil(win1_vir_width/2)\n\nYUV : ceil(win1_vir_width/4)"]
pub type Win1VirStrideR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_VIR_STRIDE` writer - Win1 Virtual stride\n\nNumber of words of Win1 yrgb Virtual width\n\nARGB888 : win1_vir_width\n\nRGB888 : (win1_vir_width*3/4) + (win1_vir_width%3)\n\nRGB565 : ceil(win1_vir_width/2)\n\nYUV : ceil(win1_vir_width/4)"]
pub type Win1VirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN1_VIR_STRIDE_UV` reader - Number of words of Win1 uv Virtual width"]
pub type Win1VirStrideUvR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_VIR_STRIDE_UV` writer - Number of words of Win1 uv Virtual width"]
pub type Win1VirStrideUvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win1 Virtual stride\n\nNumber of words of Win1 yrgb Virtual width\n\nARGB888 : win1_vir_width\n\nRGB888 : (win1_vir_width*3/4) + (win1_vir_width%3)\n\nRGB565 : ceil(win1_vir_width/2)\n\nYUV : ceil(win1_vir_width/4)"]
    #[inline(always)]
    pub fn win1_vir_stride(&self) -> Win1VirStrideR {
        Win1VirStrideR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of words of Win1 uv Virtual width"]
    #[inline(always)]
    pub fn win1_vir_stride_uv(&self) -> Win1VirStrideUvR {
        Win1VirStrideUvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win1 Virtual stride\n\nNumber of words of Win1 yrgb Virtual width\n\nARGB888 : win1_vir_width\n\nRGB888 : (win1_vir_width*3/4) + (win1_vir_width%3)\n\nRGB565 : ceil(win1_vir_width/2)\n\nYUV : ceil(win1_vir_width/4)"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vir_stride(&mut self) -> Win1VirStrideW<Win1VirSpec> {
        Win1VirStrideW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of words of Win1 uv Virtual width"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vir_stride_uv(&mut self) -> Win1VirStrideUvW<Win1VirSpec> {
        Win1VirStrideUvW::new(self, 16)
    }
}
#[doc = "win1 virtual stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_vir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_vir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1VirSpec;
impl crate::RegisterSpec for Win1VirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_vir::R`](R) reader structure"]
impl crate::Readable for Win1VirSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_vir::W`](W) writer structure"]
impl crate::Writable for Win1VirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_VIR to value 0x0140_0140"]
impl crate::Resettable for Win1VirSpec {
    const RESET_VALUE: u32 = 0x0140_0140;
}
