#[doc = "Register `EXP_H_SIZE` reader"]
pub type R = crate::R<ExpHSizeSpec>;
#[doc = "Register `EXP_H_SIZE` writer"]
pub type W = crate::W<ExpHSizeSpec>;
#[doc = "Field `isp_exp_h_size` reader - Horizontal size in pixels of one block. 35 &lt;= value &lt;=516"]
pub type IspExpHSizeR = crate::FieldReader<u16>;
#[doc = "Field `isp_exp_h_size` writer - Horizontal size in pixels of one block. 35 &lt;= value &lt;=516"]
pub type IspExpHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Horizontal size in pixels of one block. 35 &lt;= value &lt;=516"]
    #[inline(always)]
    pub fn isp_exp_h_size(&self) -> IspExpHSizeR {
        IspExpHSizeR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Horizontal size in pixels of one block. 35 &lt;= value &lt;=516"]
    #[inline(always)]
    #[must_use]
    pub fn isp_exp_h_size(&mut self) -> IspExpHSizeW<ExpHSizeSpec> {
        IspExpHSizeW::new(self, 0)
    }
}
#[doc = "Horizontal size of one block\n\nNote: exp_h_size x 5 must be less (not equal) than the horizontal size of the picture \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpHSizeSpec;
impl crate::RegisterSpec for ExpHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_h_size::R`](R) reader structure"]
impl crate::Readable for ExpHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`exp_h_size::W`](W) writer structure"]
impl crate::Writable for ExpHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXP_H_SIZE to value 0"]
impl crate::Resettable for ExpHSizeSpec {
    const RESET_VALUE: u32 = 0;
}
