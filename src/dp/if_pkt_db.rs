#[doc = "Register `IF_PKT_DB%s` reader"]
pub type R = crate::R<IfPktDbSpec>;
#[doc = "Register `IF_PKT_DB%s` writer"]
pub type W = crate::W<IfPktDbSpec>;
#[doc = "Field `IF_PKT_DB1_IF_PKT_DB25` reader - InfoFrame Packet Data Byte 1 ~ 25. The \n\nregisters define the data in the InfoFrame and \n\nthe InfoFrame type is defined by IF_TYPE."]
pub type IfPktDb1IfPktDb25R = crate::FieldReader;
#[doc = "Field `IF_PKT_DB1_IF_PKT_DB25` writer - InfoFrame Packet Data Byte 1 ~ 25. The \n\nregisters define the data in the InfoFrame and \n\nthe InfoFrame type is defined by IF_TYPE."]
pub type IfPktDb1IfPktDb25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - InfoFrame Packet Data Byte 1 ~ 25. The \n\nregisters define the data in the InfoFrame and \n\nthe InfoFrame type is defined by IF_TYPE."]
    #[inline(always)]
    pub fn if_pkt_db1_if_pkt_db25(&self) -> IfPktDb1IfPktDb25R {
        IfPktDb1IfPktDb25R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - InfoFrame Packet Data Byte 1 ~ 25. The \n\nregisters define the data in the InfoFrame and \n\nthe InfoFrame type is defined by IF_TYPE."]
    #[inline(always)]
    #[must_use]
    pub fn if_pkt_db1_if_pkt_db25(&mut self) -> IfPktDb1IfPktDb25W<IfPktDbSpec> {
        IfPktDb1IfPktDb25W::new(self, 0)
    }
}
#[doc = "InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_pkt_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_pkt_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfPktDbSpec;
impl crate::RegisterSpec for IfPktDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_pkt_db::R`](R) reader structure"]
impl crate::Readable for IfPktDbSpec {}
#[doc = "`write(|w| ..)` method takes [`if_pkt_db::W`](W) writer structure"]
impl crate::Writable for IfPktDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IF_PKT_DB%s to value 0"]
impl crate::Resettable for IfPktDbSpec {
    const RESET_VALUE: u32 = 0;
}
