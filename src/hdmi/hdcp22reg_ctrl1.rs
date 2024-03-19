#[doc = "Register `HDCP22REG_CTRL1` reader"]
pub type R = crate::R<Hdcp22regCtrl1Spec>;
#[doc = "Register `HDCP22REG_CTRL1` writer"]
pub type W = crate::W<Hdcp22regCtrl1Spec>;
#[doc = "HDCP 2.2 versus 1.4 color depth override enable:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22CdOvrEn {
    #[doc = "0: The default 1'b0 value indicates that the color depth sent to the external interface is the one configured in the vp_pr_cd.color_depth register field."]
    B0 = 0,
    #[doc = "1: Although the used color depth for pixel encoding is defined by the field vp_pr_cd.color_depth register, the color depth sent to the external interface is the one defined in register field hdcp22reg_ctrl1.hdcp22_cd_ovr_val."]
    B1 = 1,
}
impl From<Hdcp22CdOvrEn> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22CdOvrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_CD_OVR_EN` reader - HDCP 2.2 versus 1.4 color depth override enable:"]
pub type Hdcp22CdOvrEnR = crate::BitReader<Hdcp22CdOvrEn>;
impl Hdcp22CdOvrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22CdOvrEn {
        match self.bits {
            false => Hdcp22CdOvrEn::B0,
            true => Hdcp22CdOvrEn::B1,
        }
    }
    #[doc = "The default 1'b0 value indicates that the color depth sent to the external interface is the one configured in the vp_pr_cd.color_depth register field."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22CdOvrEn::B0
    }
    #[doc = "Although the used color depth for pixel encoding is defined by the field vp_pr_cd.color_depth register, the color depth sent to the external interface is the one defined in register field hdcp22reg_ctrl1.hdcp22_cd_ovr_val."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22CdOvrEn::B1
    }
}
#[doc = "Field `HDCP22_CD_OVR_EN` writer - HDCP 2.2 versus 1.4 color depth override enable:"]
pub type Hdcp22CdOvrEnW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22CdOvrEn>;
impl<'a, REG> Hdcp22CdOvrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The default 1'b0 value indicates that the color depth sent to the external interface is the one configured in the vp_pr_cd.color_depth register field."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22CdOvrEn::B0)
    }
    #[doc = "Although the used color depth for pixel encoding is defined by the field vp_pr_cd.color_depth register, the color depth sent to the external interface is the one defined in register field hdcp22reg_ctrl1.hdcp22_cd_ovr_val."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22CdOvrEn::B1)
    }
}
#[doc = "Field `HDCP22_CD_OVR_VAL` reader - HDCP color depth override value, which is sent\n\nthrough the HDCP\n\n2.2 external interface when\n\nhdcp22reg_ctrl1.hdcp22_cd_ovr_en is set.\n\nFor reference on the HDMI allowed values consult\n\nthe HDMI 1.4b specification, Table 6-1."]
pub type Hdcp22CdOvrValR = crate::FieldReader;
#[doc = "Field `HDCP22_CD_OVR_VAL` writer - HDCP color depth override value, which is sent\n\nthrough the HDCP\n\n2.2 external interface when\n\nhdcp22reg_ctrl1.hdcp22_cd_ovr_en is set.\n\nFor reference on the HDMI allowed values consult\n\nthe HDMI 1.4b specification, Table 6-1."]
pub type Hdcp22CdOvrValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 3 - HDCP 2.2 versus 1.4 color depth override enable:"]
    #[inline(always)]
    pub fn hdcp22_cd_ovr_en(&self) -> Hdcp22CdOvrEnR {
        Hdcp22CdOvrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - HDCP color depth override value, which is sent\n\nthrough the HDCP\n\n2.2 external interface when\n\nhdcp22reg_ctrl1.hdcp22_cd_ovr_en is set.\n\nFor reference on the HDMI allowed values consult\n\nthe HDMI 1.4b specification, Table 6-1."]
    #[inline(always)]
    pub fn hdcp22_cd_ovr_val(&self) -> Hdcp22CdOvrValR {
        Hdcp22CdOvrValR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 3 - HDCP 2.2 versus 1.4 color depth override enable:"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_cd_ovr_en(&mut self) -> Hdcp22CdOvrEnW<Hdcp22regCtrl1Spec> {
        Hdcp22CdOvrEnW::new(self, 3)
    }
    #[doc = "Bits 4:7 - HDCP color depth override value, which is sent\n\nthrough the HDCP\n\n2.2 external interface when\n\nhdcp22reg_ctrl1.hdcp22_cd_ovr_en is set.\n\nFor reference on the HDMI allowed values consult\n\nthe HDMI 1.4b specification, Table 6-1."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_cd_ovr_val(&mut self) -> Hdcp22CdOvrValW<Hdcp22regCtrl1Spec> {
        Hdcp22CdOvrValW::new(self, 4)
    }
}
#[doc = "HDCP 2.2 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regCtrl1Spec;
impl crate::RegisterSpec for Hdcp22regCtrl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_ctrl1::R`](R) reader structure"]
impl crate::Readable for Hdcp22regCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hdcp22reg_ctrl1::W`](W) writer structure"]
impl crate::Writable for Hdcp22regCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP22REG_CTRL1 to value 0"]
impl crate::Resettable for Hdcp22regCtrl1Spec {
    const RESET_VALUE: u8 = 0;
}
