#[doc = "Register `SWREG47_SAO_CTU_POSITION` reader"]
pub type R = crate::R<Swreg47SaoCtuPositionSpec>;
#[doc = "Register `SWREG47_SAO_CTU_POSITION` writer"]
pub type W = crate::W<Swreg47SaoCtuPositionSpec>;
#[doc = "Field `SW_SAOWR_XOFFET` reader - saowr x address offset\n\nsaowr x address offset, its unit is 128bit"]
pub type SwSaowrXoffetR = crate::FieldReader<u16>;
#[doc = "Field `SW_SAOWR_XOFFET` writer - saowr x address offset\n\nsaowr x address offset, its unit is 128bit"]
pub type SwSaowrXoffetW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_SAOWR_YOFFSET` reader - saowr y offset\n\nsaowr y offset , its unit is 4 pixels"]
pub type SwSaowrYoffsetR = crate::FieldReader<u16>;
#[doc = "Field `SW_SAOWR_YOFFSET` writer - saowr y offset\n\nsaowr y offset , its unit is 4 pixels"]
pub type SwSaowrYoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:8 - saowr x address offset\n\nsaowr x address offset, its unit is 128bit"]
    #[inline(always)]
    pub fn sw_saowr_xoffet(&self) -> SwSaowrXoffetR {
        SwSaowrXoffetR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:25 - saowr y offset\n\nsaowr y offset , its unit is 4 pixels"]
    #[inline(always)]
    pub fn sw_saowr_yoffset(&self) -> SwSaowrYoffsetR {
        SwSaowrYoffsetR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - saowr x address offset\n\nsaowr x address offset, its unit is 128bit"]
    #[inline(always)]
    #[must_use]
    pub fn sw_saowr_xoffet(&mut self) -> SwSaowrXoffetW<Swreg47SaoCtuPositionSpec> {
        SwSaowrXoffetW::new(self, 0)
    }
    #[doc = "Bits 16:25 - saowr y offset\n\nsaowr y offset , its unit is 4 pixels"]
    #[inline(always)]
    #[must_use]
    pub fn sw_saowr_yoffset(&mut self) -> SwSaowrYoffsetW<Swreg47SaoCtuPositionSpec> {
        SwSaowrYoffsetW::new(self, 16)
    }
}
#[doc = "sao ctu position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg47_sao_ctu_position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg47_sao_ctu_position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg47SaoCtuPositionSpec;
impl crate::RegisterSpec for Swreg47SaoCtuPositionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg47_sao_ctu_position::R`](R) reader structure"]
impl crate::Readable for Swreg47SaoCtuPositionSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg47_sao_ctu_position::W`](W) writer structure"]
impl crate::Writable for Swreg47SaoCtuPositionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG47_SAO_CTU_POSITION to value 0"]
impl crate::Resettable for Swreg47SaoCtuPositionSpec {
    const RESET_VALUE: u32 = 0;
}
