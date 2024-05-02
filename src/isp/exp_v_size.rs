#[doc = "Register `EXP_V_SIZE` reader"]
pub type R = crate::R<ExpVSizeSpec>;
#[doc = "Register `EXP_V_SIZE` writer"]
pub type W = crate::W<ExpVSizeSpec>;
#[doc = "Field `isp_exp_v_size` reader - Vertical size in pixels of one block. 28 &lt;= value &lt;=390"]
pub type IspExpVSizeR = crate::FieldReader<u16>;
#[doc = "Field `isp_exp_v_size` writer - Vertical size in pixels of one block. 28 &lt;= value &lt;=390"]
pub type IspExpVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Vertical size in pixels of one block. 28 &lt;= value &lt;=390"]
    #[inline(always)]
    pub fn isp_exp_v_size(&self) -> IspExpVSizeR {
        IspExpVSizeR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical size in pixels of one block. 28 &lt;= value &lt;=390"]
    #[inline(always)]
    #[must_use]
    pub fn isp_exp_v_size(&mut self) -> IspExpVSizeW<ExpVSizeSpec> {
        IspExpVSizeW::new(self, 0)
    }
}
#[doc = "Vertical size of one block\n\nNote: The vertical size must be set in a way that after the last measurement window at \n\n\n\nleast two lines of the image will follow. In addition only even values for vertical size are \n\n\n\nallowed (vertical size must be a multiple of 2). \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpVSizeSpec;
impl crate::RegisterSpec for ExpVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_v_size::R`](R) reader structure"]
impl crate::Readable for ExpVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`exp_v_size::W`](W) writer structure"]
impl crate::Writable for ExpVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXP_V_SIZE to value 0"]
impl crate::Resettable for ExpVSizeSpec {
    const RESET_VALUE: u32 = 0;
}
