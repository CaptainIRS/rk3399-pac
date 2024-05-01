#[doc = "Register `WIN3_VIR0_1` reader"]
pub type R = crate::R<Win3Vir0_1Spec>;
#[doc = "Register `WIN3_VIR0_1` writer"]
pub type W = crate::W<Win3Vir0_1Spec>;
#[doc = "Field `WIN3_VIR_STRIDE0` reader - Win3 Virtual stride0\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
pub type Win3VirStride0R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_VIR_STRIDE0` writer - Win3 Virtual stride0\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
pub type Win3VirStride0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN3_VIR_STRIDE1` reader - Win3 Virtual stride1\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
pub type Win3VirStride1R = crate::FieldReader<u16>;
#[doc = "Field `WIN3_VIR_STRIDE1` writer - Win3 Virtual stride1\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
pub type Win3VirStride1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win3 Virtual stride0\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
    #[inline(always)]
    pub fn win3_vir_stride0(&self) -> Win3VirStride0R {
        Win3VirStride0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win3 Virtual stride1\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
    #[inline(always)]
    pub fn win3_vir_stride1(&self) -> Win3VirStride1R {
        Win3VirStride1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win3 Virtual stride0\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
    #[inline(always)]
    #[must_use]
    pub fn win3_vir_stride0(&mut self) -> Win3VirStride0W<Win3Vir0_1Spec> {
        Win3VirStride0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win3 Virtual stride1\n\nNumber of words of Win3 Virtual1 width\n\nARGB888 : win3_vir_width1\n\nRGB888 : (win3_vir_width1 * 3/4) + (win3_vir_width1 % 3)\n\nRGB565 : ceil(win3_vir_width1 / 2)\n\n8BPP : ceil(win3_vir_width1 / 4)\n\n4BPP : ceil(win3_vir_width1 / 8)\n\n2BPP : ceil(win3_vir_width1 / 16)\n\n1BPP : ceil(win3_vir_width1 / 32)"]
    #[inline(always)]
    #[must_use]
    pub fn win3_vir_stride1(&mut self) -> Win3VirStride1W<Win3Vir0_1Spec> {
        Win3VirStride1W::new(self, 16)
    }
}
#[doc = "Win3 virtual stride0 and virtaul stride1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_vir0_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_vir0_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Vir0_1Spec;
impl crate::RegisterSpec for Win3Vir0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_vir0_1::R`](R) reader structure"]
impl crate::Readable for Win3Vir0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_vir0_1::W`](W) writer structure"]
impl crate::Writable for Win3Vir0_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_VIR0_1 to value 0x0140_0140"]
impl crate::Resettable for Win3Vir0_1Spec {
    const RESET_VALUE: u32 = 0x0140_0140;
}
