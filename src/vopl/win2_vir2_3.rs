#[doc = "Register `WIN2_VIR2_3` reader"]
pub type R = crate::R<Win2Vir2_3Spec>;
#[doc = "Register `WIN2_VIR2_3` writer"]
pub type W = crate::W<Win2Vir2_3Spec>;
#[doc = "Field `WIN2_VIR_STRIDE2` reader - Win2 Virtual stride2\n\nNumber of words of Win2 Virtual2 width\n\nARGB888 : win2_vir_width2\n\nRGB888 : (win2_vir_width2 * 3/4) + (win2_vir_width2 % 3)\n\nRGB565 : ceil(win2_vir_width2 / 2)\n\n8BPP : ceil(win2_vir_width2 / 4)\n\n4BPP : ceil(win2_vir_width2 / 8)\n\n2BPP : ceil(win2_vir_width2 / 16)\n\n1BPP : ceil(win1_vir_width2 / 32)"]
pub type Win2VirStride2R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_VIR_STRIDE2` writer - Win2 Virtual stride2\n\nNumber of words of Win2 Virtual2 width\n\nARGB888 : win2_vir_width2\n\nRGB888 : (win2_vir_width2 * 3/4) + (win2_vir_width2 % 3)\n\nRGB565 : ceil(win2_vir_width2 / 2)\n\n8BPP : ceil(win2_vir_width2 / 4)\n\n4BPP : ceil(win2_vir_width2 / 8)\n\n2BPP : ceil(win2_vir_width2 / 16)\n\n1BPP : ceil(win1_vir_width2 / 32)"]
pub type Win2VirStride2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN2_VIR_STRIDE3` reader - Win2 Virtual stride3\n\nNumber of words of Win2 Virtual3 width\n\nARGB888 : win2_vir_width3\n\nRGB888 : (win2_vir_width3 * 3/4) + (win2_vir_width3 % 3)\n\nRGB565 : ceil(win2_vir_width3 / 2)\n\n8BPP : ceil(win2_vir_width3 / 4)\n\n4BPP : ceil(win2_vir_width3 / 8)\n\n2BPP : ceil(win2_vir_width3 / 16)\n\n1BPP : ceil(win1_vir_width3 / 32)"]
pub type Win2VirStride3R = crate::FieldReader<u16>;
#[doc = "Field `WIN2_VIR_STRIDE3` writer - Win2 Virtual stride3\n\nNumber of words of Win2 Virtual3 width\n\nARGB888 : win2_vir_width3\n\nRGB888 : (win2_vir_width3 * 3/4) + (win2_vir_width3 % 3)\n\nRGB565 : ceil(win2_vir_width3 / 2)\n\n8BPP : ceil(win2_vir_width3 / 4)\n\n4BPP : ceil(win2_vir_width3 / 8)\n\n2BPP : ceil(win2_vir_width3 / 16)\n\n1BPP : ceil(win1_vir_width3 / 32)"]
pub type Win2VirStride3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win2 Virtual stride2\n\nNumber of words of Win2 Virtual2 width\n\nARGB888 : win2_vir_width2\n\nRGB888 : (win2_vir_width2 * 3/4) + (win2_vir_width2 % 3)\n\nRGB565 : ceil(win2_vir_width2 / 2)\n\n8BPP : ceil(win2_vir_width2 / 4)\n\n4BPP : ceil(win2_vir_width2 / 8)\n\n2BPP : ceil(win2_vir_width2 / 16)\n\n1BPP : ceil(win1_vir_width2 / 32)"]
    #[inline(always)]
    pub fn win2_vir_stride2(&self) -> Win2VirStride2R {
        Win2VirStride2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win2 Virtual stride3\n\nNumber of words of Win2 Virtual3 width\n\nARGB888 : win2_vir_width3\n\nRGB888 : (win2_vir_width3 * 3/4) + (win2_vir_width3 % 3)\n\nRGB565 : ceil(win2_vir_width3 / 2)\n\n8BPP : ceil(win2_vir_width3 / 4)\n\n4BPP : ceil(win2_vir_width3 / 8)\n\n2BPP : ceil(win2_vir_width3 / 16)\n\n1BPP : ceil(win1_vir_width3 / 32)"]
    #[inline(always)]
    pub fn win2_vir_stride3(&self) -> Win2VirStride3R {
        Win2VirStride3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win2 Virtual stride2\n\nNumber of words of Win2 Virtual2 width\n\nARGB888 : win2_vir_width2\n\nRGB888 : (win2_vir_width2 * 3/4) + (win2_vir_width2 % 3)\n\nRGB565 : ceil(win2_vir_width2 / 2)\n\n8BPP : ceil(win2_vir_width2 / 4)\n\n4BPP : ceil(win2_vir_width2 / 8)\n\n2BPP : ceil(win2_vir_width2 / 16)\n\n1BPP : ceil(win1_vir_width2 / 32)"]
    #[inline(always)]
    #[must_use]
    pub fn win2_vir_stride2(&mut self) -> Win2VirStride2W<Win2Vir2_3Spec> {
        Win2VirStride2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win2 Virtual stride3\n\nNumber of words of Win2 Virtual3 width\n\nARGB888 : win2_vir_width3\n\nRGB888 : (win2_vir_width3 * 3/4) + (win2_vir_width3 % 3)\n\nRGB565 : ceil(win2_vir_width3 / 2)\n\n8BPP : ceil(win2_vir_width3 / 4)\n\n4BPP : ceil(win2_vir_width3 / 8)\n\n2BPP : ceil(win2_vir_width3 / 16)\n\n1BPP : ceil(win1_vir_width3 / 32)"]
    #[inline(always)]
    #[must_use]
    pub fn win2_vir_stride3(&mut self) -> Win2VirStride3W<Win2Vir2_3Spec> {
        Win2VirStride3W::new(self, 16)
    }
}
#[doc = "Win2 virtual stride2 and virtaul stride3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_vir2_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_vir2_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Vir2_3Spec;
impl crate::RegisterSpec for Win2Vir2_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_vir2_3::R`](R) reader structure"]
impl crate::Readable for Win2Vir2_3Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_vir2_3::W`](W) writer structure"]
impl crate::Writable for Win2Vir2_3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_VIR2_3 to value 0x0140_0140"]
impl crate::Resettable for Win2Vir2_3Spec {
    const RESET_VALUE: u32 = 0x0140_0140;
}
